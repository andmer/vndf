use nalgebra::Pnt2;

use client::graphics::SHIP_SIZE;


pub enum CollideKind {
    Sphere,
    Rect,
}

pub struct Collider {
    points: [Pnt2<f32>;4], // bounding box
    kind: CollideKind,
}

impl Collider {
    pub fn new (points: [Pnt2<f32>;4], kind: CollideKind) -> Collider {
	Collider { points: points,
		   kind: kind, }
    }

    /// builds based on current ship mesh layout (from equilateral triangle)
    pub fn new_from_ship () -> Collider {
	let size = SHIP_SIZE/2.0;
	let p = [Pnt2::new(-0.5,-0.5) * size,
		 Pnt2::new(0.5,-0.5) * size,
		 Pnt2::new(0.5,0.5) * size,
		 Pnt2::new(-0.5,0.5) * size,];
	
	Collider::new(p,CollideKind::Rect)
    }

    pub fn check_collision (&self, other: &Collider) -> bool {
	false
    }
}