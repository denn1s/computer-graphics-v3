use crate::framebuffer::Framebuffer;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, ox: usize, oy: usize, a: f32, block_size: usize) {
  let mut c = 0 as f32;

  loop {
    // todo: simplify this casting logic
    let x = (ox as f32 + c * a.cos()) as usize;
    let y = (oy as f32 + c * a.sin()) as usize;

    let i = x / block_size;
    let j = y / block_size;

    if maze[j][i] != ' ' {
      break;
    }

    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(x, y);

    c += 10.0;
  }
}


