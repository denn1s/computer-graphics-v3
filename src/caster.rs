use crate::framebuffer::Framebuffer;

pub struct Intersect {
  pub distance: f64,
  pub impact: char
}

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, ox: usize, oy: usize, a: f64, block_size: usize) -> Intersect {
  let mut c = 0 as f64;

  loop {
    // todo: simplify this casting logic
    let x = ox as f64 + c * a.cos() as f64;
    let y = oy as f64 + c * a.sin() as f64;

    let i = (x / block_size as f64) as usize;
    let j = (y / block_size as f64) as usize;

    if maze[j][i] != ' ' {
      return Intersect{
        distance: c, 
        impact: maze[j][i]
      };
    }

    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(x as usize, y as usize);

    // c += 10.0;
    c += 0.05;
  }
}


