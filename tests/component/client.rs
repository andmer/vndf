use common::testing::{
	Client,
	MockGameService
};
use common::physics::{
	Body,
	Radians,
	Vec2
};
use common::protocol::{
	Perception,
	Ship
};


#[test]
fn it_should_interpolate_between_perceptions() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port);

	game_service.accept_client();

	let pos_1 = Vec2::zero();
	let pos_2 = Vec2(10.0, 0.0);

	let perception_1 = Perception {
		self_id: 0,
		ships  : vec!(Ship {
			id  : 0,
			body: Body {
				position: pos_1,
				velocity: Vec2(10.0, 0.0),
				attitude: Radians(0.0)
			}
		})
	};
	let mut perception_2 = perception_1.clone();
	perception_2.ships.get_mut(0).body.position = pos_2;

	game_service.send_perception(&perception_1);
	game_service.send_perception(&perception_2);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	while frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	while frame_1.ships.get(0).position == pos_1 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	assert!(is_on_line(
		pos_1,
		pos_2,
		frame_1.ships.get(0).position));
	assert!(is_on_line(
		pos_1,
		pos_2,
		frame_2.ships.get(0).position));
	assert!(frame_2.ships.get(0).position != pos_2);
}

fn is_on_line(Vec2(x1, y1): Vec2, Vec2(x2, y2): Vec2, Vec2(px, py): Vec2) -> bool {
	((x2 - x1) * (py - y1) - (y2 - y1) * (px - x1)) == 0.0
}
