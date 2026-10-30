#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <algorithm>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

constexpr int WINDOW_WIDTH = 1200;
constexpr int WINDOW_HEIGHT = 800;

std::string read_file(const std::string& path) {
    std::ifstream file(path);
    if (!file) throw std::runtime_error("Cannot open " + path + "; run from the repository root");
    return {std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()};
}

GLuint compile_shader(GLenum type, const std::string& path) {
    std::string source = read_file(path);
    const char* text = source.c_str();
    GLuint shader = glCreateShader(type);
    glShaderSource(shader, 1, &text, nullptr);
    glCompileShader(shader);
    GLint compiled = GL_FALSE;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &compiled);
    if (!compiled) {
        GLint length = 0;
        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &length);
        std::string log(length, '\0');
        glGetShaderInfoLog(shader, length, nullptr, log.data());
        glDeleteShader(shader);
        throw std::runtime_error(path + "\n" + log);
    }
    return shader;
}

GLuint create_program() {
    GLuint vertex = compile_shader(GL_VERTEX_SHADER, "gpu_demo/sphere.vert");
    GLuint fragment = compile_shader(GL_FRAGMENT_SHADER, "gpu_demo/sun.frag");
    GLuint program = glCreateProgram();
    glAttachShader(program, vertex);
    glAttachShader(program, fragment);
    glLinkProgram(program);
    glDeleteShader(vertex);
    glDeleteShader(fragment);
    GLint linked = GL_FALSE;
    glGetProgramiv(program, GL_LINK_STATUS, &linked);
    if (!linked) {
        GLint length = 0;
        glGetProgramiv(program, GL_INFO_LOG_LENGTH, &length);
        std::string log(length, '\0');
        glGetProgramInfoLog(program, length, nullptr, log.data());
        glDeleteProgram(program);
        throw std::runtime_error(log);
    }
    return program;
}

std::vector<float> load_sphere() {
    std::istringstream file(read_file("assets/models/sphere.obj"));
    std::vector<glm::vec3> positions;
    std::vector<float> vertices;
    std::string line;
    while (std::getline(file, line)) {
        std::istringstream row(line);
        std::string kind;
        row >> kind;
        if (kind == "v") {
            glm::vec3 position;
            row >> position.x >> position.y >> position.z;
            positions.push_back(position);
        } else if (kind == "f") {
            std::vector<int> face;
            std::string token;
            while (row >> token) {
                int index = std::stoi(token.substr(0, token.find('/')));
                face.push_back(index > 0 ? index - 1 : static_cast<int>(positions.size()) + index);
            }
            for (size_t i = 1; i + 1 < face.size(); ++i) {
                for (int index : {face[0], face[i], face[i + 1]}) {
                    const auto& p = positions.at(index);
                    vertices.insert(vertices.end(), {p.x, p.y, p.z});
                }
            }
        }
    }
    return vertices;
}

void save_frame(const std::string& path, int width, int height) {
    std::vector<unsigned char> pixels(width * height * 3);
    glPixelStorei(GL_PACK_ALIGNMENT, 1);
    glReadPixels(0, 0, width, height, GL_RGB, GL_UNSIGNED_BYTE, pixels.data());
    std::ofstream file(path, std::ios::binary);
    if (!file) throw std::runtime_error("Cannot save " + path);
    file << "P6\n" << width << ' ' << height << "\n255\n";
    for (int y = height - 1; y >= 0; --y) {
        file.write(reinterpret_cast<char*>(pixels.data() + y * width * 3), width * 3);
    }
}

void run(GLFWwindow* window, int frame_limit, const std::string& capture, float fixed_time) {
    std::cout << "Renderer: " << glGetString(GL_RENDERER) << '\n'
              << "OpenGL: " << glGetString(GL_VERSION) << '\n';
    auto vertices = load_sphere();
    std::cout << "Sphere: " << vertices.size() / 9 << " triangles\n";
    GLuint program = create_program();
    GLuint vertex_array, vertex_buffer;
    glGenVertexArrays(1, &vertex_array);
    glGenBuffers(1, &vertex_buffer);
    glBindVertexArray(vertex_array);
    glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer);
    glBufferData(GL_ARRAY_BUFFER, vertices.size() * sizeof(float), vertices.data(), GL_STATIC_DRAW);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), nullptr);
    glEnableVertexAttribArray(0);
    glEnable(GL_DEPTH_TEST);
    glDisable(GL_DITHER);
    bool paused = false, rotating = false;
    bool was_space = false, was_r = false, was_reload = false;
    float time = 0.0f, angle = 0.0f;
    double previous = glfwGetTime(), report_start = previous;
    int frames = 0, report_frames = 0;
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        double now = glfwGetTime();
        float delta = static_cast<float>(now - previous);
        previous = now;
        bool space = glfwGetKey(window, GLFW_KEY_SPACE) == GLFW_PRESS;
        bool rotate = glfwGetKey(window, GLFW_KEY_R) == GLFW_PRESS;
        bool reload = glfwGetKey(window, GLFW_KEY_F5) == GLFW_PRESS;
        if (space && !was_space) paused = !paused;
        if (rotate && !was_r) rotating = !rotating;
        if (reload && !was_reload) {
            try {
                GLuint replacement = create_program();
                glDeleteProgram(program);
                program = replacement;
                std::cout << "Shaders reloaded\n";
            } catch (const std::exception& error) {
                std::cerr << error.what() << '\n';
            }
        }
        was_space = space; was_r = rotate; was_reload = reload;
        if (!paused) {
            time += delta;
            if (rotating) angle += delta * 0.35f;
        }
        if (glfwGetKey(window, GLFW_KEY_HOME) == GLFW_PRESS) { time = 0.0f; angle = 0.0f; }
        if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) break;
        int width, height;
        glfwGetFramebufferSize(window, &width, &height);
        if (width == 0 || height == 0) continue;
        glViewport(0, 0, width, height);
        glClearColor(0.04f, 0.08f, 0.18f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glUseProgram(program);
        glm::mat4 model = glm::rotate(glm::mat4(1.0f), angle, glm::vec3(0, 1, 0));
        glm::mat4 view = glm::lookAt(glm::vec3(0, 0, 3.2f), glm::vec3(0), glm::vec3(0, 1, 0));
        glm::mat4 projection = glm::perspective(glm::radians(60.0f), float(width) / height, 0.1f, 100.0f);
        glUniformMatrix4fv(glGetUniformLocation(program, "model"), 1, GL_FALSE, glm::value_ptr(model));
        glUniformMatrix4fv(glGetUniformLocation(program, "view"), 1, GL_FALSE, glm::value_ptr(view));
        glUniformMatrix4fv(glGetUniformLocation(program, "projection"), 1, GL_FALSE, glm::value_ptr(projection));
        glUniform1f(glGetUniformLocation(program, "time"), fixed_time >= 0.0f ? fixed_time : time);
        glDrawArrays(GL_TRIANGLES, 0, static_cast<GLsizei>(vertices.size() / 3));
        if (!capture.empty() && frames == 0) save_frame(capture, width, height);
        glfwSwapBuffers(window);
        ++frames; ++report_frames;
        double elapsed = glfwGetTime() - report_start;
        if (elapsed >= 0.5) {
            std::ostringstream title;
            title << "GPU / GLSL sun | " << std::fixed << std::setprecision(0) << report_frames / elapsed
                  << " FPS | " << std::setprecision(2) << elapsed * 1000.0 / report_frames
                  << " ms/frame | Space: pause  R: rotate  F5: reload";
            glfwSetWindowTitle(window, title.str().c_str());
            report_start = glfwGetTime(); report_frames = 0;
        }
        if (frame_limit > 0 && frames >= frame_limit) break;
    }
    glDeleteProgram(program);
    glDeleteBuffers(1, &vertex_buffer);
    glDeleteVertexArrays(1, &vertex_array);
}

int main(int argc, char** argv) {
    try {
        int frame_limit = 0;
        float fixed_time = -1.0f;
        std::string capture;
        for (int i = 1; i + 1 < argc; ++i) {
            std::string arg = argv[i];
            if (arg == "--frames") frame_limit = std::stoi(argv[++i]);
            else if (arg == "--capture") capture = argv[++i];
            else if (arg == "--time") fixed_time = std::stof(argv[++i]);
        }
        glfwSetErrorCallback([](int, const char* message) { std::cerr << message << '\n'; });
        if (!glfwInit()) throw std::runtime_error("GLFW initialization failed");
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);
        GLFWwindow* window = glfwCreateWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "GPU / GLSL sun", nullptr, nullptr);
        if (!window) throw std::runtime_error("OpenGL 3.3 context unavailable");
        glfwMakeContextCurrent(window);
        glfwSwapInterval(0);
        glewExperimental = GL_TRUE;
        if (glewInit() != GLEW_OK) throw std::runtime_error("GLEW initialization failed");
        run(window, frame_limit, capture, fixed_time);
        glfwDestroyWindow(window);
        glfwTerminate();
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        glfwTerminate();
        return 1;
    }
}
