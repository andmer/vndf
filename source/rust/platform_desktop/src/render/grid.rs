use cgmath::{
	FixedArray,
	Vector3,
};
use gfx::{
	mod,
	DeviceHelper,
	Frame,
	ToSlice,
};

use platform::Camera;

use super::{
	camera_to_transform,
	Graphics,
	Transform,
	Vertex,
};


static VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4 transform;

		in vec3 position;

		void main() {
			gl_Position = transform * vec4(position, 1.0);
		}
	"
};

static FRAGMENT_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		out vec4 out_color;

		void main() {
			out_color = vec4(1.0, 1.0, 1.0, 1.0);
		}
	"
};


#[shader_param(GridBatch)]
struct Params {
	transform: [[f32, ..4], ..4],
}


pub struct Grid {
	batch: GridBatch,
}

impl Grid {
	pub fn new(graphics: &mut Graphics) -> Grid {
		let grid_data = [
			Vertex::without_tex([ -700.0, -600.0, 0.0 ]),
			Vertex::without_tex([ -700.0,  600.0, 0.0 ]),
			Vertex::without_tex([ -500.0, -600.0, 0.0 ]),
			Vertex::without_tex([ -500.0,  600.0, 0.0 ]),
			Vertex::without_tex([ -300.0, -600.0, 0.0 ]),
			Vertex::without_tex([ -300.0,  600.0, 0.0 ]),
			Vertex::without_tex([ -100.0, -600.0, 0.0 ]),
			Vertex::without_tex([ -100.0,  600.0, 0.0 ]),
			Vertex::without_tex([  100.0, -600.0, 0.0 ]),
			Vertex::without_tex([  100.0,  600.0, 0.0 ]),
			Vertex::without_tex([  300.0, -600.0, 0.0 ]),
			Vertex::without_tex([  300.0,  600.0, 0.0 ]),
			Vertex::without_tex([  500.0, -600.0, 0.0 ]),
			Vertex::without_tex([  500.0,  600.0, 0.0 ]),
			Vertex::without_tex([  700.0, -600.0, 0.0 ]),
			Vertex::without_tex([  700.0,  600.0, 0.0 ]),

			Vertex::without_tex([ -700.0, -600.0, 0.0 ]),
			Vertex::without_tex([  700.0, -600.0, 0.0 ]),
			Vertex::without_tex([ -700.0, -400.0, 0.0 ]),
			Vertex::without_tex([  700.0, -400.0, 0.0 ]),
			Vertex::without_tex([ -700.0, -200.0, 0.0 ]),
			Vertex::without_tex([  700.0, -200.0, 0.0 ]),
			Vertex::without_tex([ -700.0,    0.0, 0.0 ]),
			Vertex::without_tex([  700.0,    0.0, 0.0 ]),
			Vertex::without_tex([ -700.0,  200.0, 0.0 ]),
			Vertex::without_tex([  700.0,  200.0, 0.0 ]),
			Vertex::without_tex([ -700.0,  400.0, 0.0 ]),
			Vertex::without_tex([  700.0,  400.0, 0.0 ]),
			Vertex::without_tex([ -700.0,  600.0, 0.0 ]),
			Vertex::without_tex([  700.0,  600.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(grid_data);
		let slice = mesh.to_slice(gfx::Line);

		let program = graphics.device
			.link_program(
				VERTEX_SHADER.clone(),
				FRAGMENT_SHADER.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				&gfx::DrawState::new().blend(gfx::BlendAlpha)
			)
			.unwrap();

		Grid {
			batch: batch,
		}
	}

	pub fn draw(
		&self,
		graphics  : &mut Graphics,
		frame     : &Frame,
		camera    : &Camera,
		projection: Transform,
	) {
		let grid_camera = Camera {
			center: Vector3::new(
				camera.center[0] % 200.0,
				camera.center[1] % 200.0,
				camera.center[2],
			),

			perspective: camera.perspective,
			distance   : camera.distance,
		};

		let view = camera_to_transform(&grid_camera);

		let params = Params {
			transform: projection.mul(&view).into_fixed(),
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}
