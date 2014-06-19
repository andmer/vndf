use std::ascii::StrAsciiExt;
use std::collections::HashMap;
use std::gc::Gc;
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::parse::token;

use parse;


pub fn items(context: &ExtCtxt, ecs: &parse::ECS) -> Vec<Gc<ast::Item>> {
	let components: HashMap<String, Component> = ecs.components
		.iter()
		.map(|component| {
			let component = Component::generate(context, component);
			(component.name.clone(), component)
		})
		.collect();

	let entities: Vec<Entity> = ecs.entities
		.iter()
		.map(|entity|
			Entity::generate(context, entity, &components))
		.collect();

	let worlds: Vec<World> = ecs.worlds
		.iter()
		.map(|world|
			World::generate(context, world, &entities))
		.collect();

	let mut items = Vec::new();
	for &World(ref world) in worlds.iter() {
		items.push_all(world.as_slice());
	}

	items
}


#[deriving(Clone)]
pub struct Component {
	name: String,

	var_name   : ast::Ident,
	decl       : Vec<ast::TokenTree>,
	init       : Vec<ast::TokenTree>,
	import     : Vec<ast::TokenTree>,
	insert     : Vec<ast::TokenTree>,
	remove     : Vec<ast::TokenTree>,
	entity_decl: Vec<ast::TokenTree>,
	entity_init: Vec<ast::TokenTree>,
}

impl Component {
	fn generate(context: &ExtCtxt, component: &parse::Component) -> Component {
		let var_name = ast::Ident::new(
			token::intern(ident_to_lower(component.name).as_slice()));

		let collection = component.collection;
		let ty         = component.ty;

		let decl = quote_tokens!(&*context,
			pub $collection: ::rustecs::Components<$ty>,
		);

		let init = quote_tokens!(&*context,
			$collection: ::rustecs::components(),
		);

		let import = quote_tokens!(&*context,
			match entity.$var_name {
				Some(component) => {
					let _ = world.$collection.insert(entity.id, component);
				},
				None =>
					()
			}
		);

		let insert = quote_tokens!(&*context,
			self.$collection.insert(id, $var_name);
		);

		let remove = quote_tokens!(&*context,
			self.$collection.remove(&id);
		);

		let entity_decl = quote_tokens!(&*context,
			pub $var_name: Option<$ty>,
		);

		let entity_init = quote_tokens!(&*context,
			$var_name: self.$collection.find_copy(id),
		);

		Component {
			name       : token::get_ident(component.name).to_str(),
			var_name   : var_name,
			decl       : decl,
			init       : init,
			import     : import,
			insert     : insert,
			remove     : remove,
			entity_decl: entity_decl,
			entity_init: entity_init,
		}
	}
}


#[deriving(Clone)]
pub struct Entity {
	name      : ast::Ident,
	components: HashMap<String, Component>,
	create_fn : Vec<ast::TokenTree>,
	import_fn : Vec<ast::TokenTree>,
}

impl Entity {
	fn generate(
		context       : &ExtCtxt,
		entity        : &parse::Entity,
		all_components: &HashMap<String, Component>
	) -> Entity {
		let entity_components = entity.components
			.iter()
			.map(|&ident| {
				let name = token::get_ident(ident).to_str();
				(name.clone(), (*all_components.get(&name)).clone())
			})
			.collect();

		let ordered_components: Vec<String> = entity.components
			.iter()
			.map(|&ident|
				token::get_ident(ident).to_str())
			.collect();

		let create_fn = Entity::create_fn(
			context,
			entity,
			&entity_components,
			&ordered_components);

		let import_fn = Entity::import_fn(
			context,
			entity,
			&entity_components,
			&ordered_components);

		Entity {
			name      : entity.name,
			components: entity_components,
			create_fn : create_fn,
			import_fn : import_fn,
		}
	}

	fn create_fn(
		context           : &ExtCtxt,
		entity            : &parse::Entity,
		components        : &HashMap<String, Component>,
		ordered_components: &Vec<String>
	) -> Vec<ast::TokenTree> {
		let name = ast::Ident::new(token::intern(
			"create_"
				.to_str()
				.append(ident_to_lower(entity.name).as_slice())
				.as_slice()));

		let mut args = Vec::new();
		for (i, arg) in entity.args.iter().enumerate() {
			if i + 1 < entity.args.len() {
				args.push_all(quote_tokens!(&*context, $arg,).as_slice());
			}
			else {
				args.push_all(quote_tokens!(&*context, $arg).as_slice());
			}
		}

		let mut component_names = Vec::new();
		for (i, name) in ordered_components.iter().enumerate() {
			let component = components.get(name);
			let var_name  = component.var_name;

			if i + 1 < components.len() {
				component_names.push_all(
					quote_tokens!(&*context, $var_name,).as_slice());
			}
			else {
				component_names.push_all(
					quote_tokens!(&*context, $var_name).as_slice());
			}
		}

		let init_block = entity.init_block;

		let mut inserts = Vec::new();
		for (_, component) in components.iter() {
			inserts.push_all(component.insert.as_slice());
		}

		quote_tokens!(&*context,
			pub fn $name(&mut self, $args) -> ::rustecs::EntityId {
				let id = self.next_id;
				self.next_id += 1;

				let ($component_names) = $init_block;

				self.entities.insert(id);
				$inserts

				id
			}
		)
	}

	fn import_fn(
		context           : &ExtCtxt,
		entity            : &parse::Entity,
		components        : &HashMap<String, Component>,
		ordered_components: &Vec<String>
	) -> Vec<ast::TokenTree> {
		let name = ast::Ident::new(token::intern(
			"import_"
				.to_str()
				.append(ident_to_lower(entity.name).as_slice())
				.as_slice()));

		let mut args = Vec::new();
		for (i, arg) in entity.args.iter().enumerate() {
			if i + 1 < entity.args.len() {
				args.push_all(quote_tokens!(&*context, $arg,).as_slice());
			}
			else {
				args.push_all(quote_tokens!(&*context, $arg).as_slice());
			}
		}

		let mut component_names = Vec::new();
		for (i, name) in ordered_components.iter().enumerate() {
			let component = components.get(name);
			let var_name  = component.var_name;

			if i + 1 < components.len() {
				component_names.push_all(
					quote_tokens!(&*context, $var_name,).as_slice());
			}
			else {
				component_names.push_all(
					quote_tokens!(&*context, $var_name).as_slice());
			}
		}

		let init_block = entity.init_block;

		let mut inserts = Vec::new();
		for (_, component) in components.iter() {
			inserts.push_all(component.insert.as_slice());
		}

		quote_tokens!(&*context,
			pub fn $name(&mut self, id: ::rustecs::EntityId, $args) -> ::rustecs::EntityId {
				if id >= self.next_id {
					self.next_id = id + 1;
				}

				let ($component_names) = $init_block;

				self.entities.insert(id);
				$inserts

				id
			}
		)
	}
}


struct World(Vec<Gc<ast::Item>>);

impl World {
	fn generate(
		context     : &ExtCtxt,
		world       : &parse::World,
		all_entities: &Vec<Entity>
	) -> World {
		let entities   = World::entities(&world.entities, all_entities);
		let components = World::components(&entities);

		let name         = world.name;
		let decls        = World::component_decls(&components);
		let inits        = World::component_inits(&components);
		let imports      = World::imports(&components);
		let create_fns   = World::create_fns(&entities);
		let import_fns   = World::import_fns(&entities);
		let removes      = World::removes(&components);
		let entity_name  = World::entity_name(world.name);
		let entity_decls = World::entity_decls(&components);
		let entity_init  = World::entity_init(&components);

		let structure = quote_item!(&*context,
			#[deriving(Show)]
			pub struct $name {
				entities: ::std::collections::HashSet<::rustecs::EntityId>,
				next_id : ::rustecs::EntityId,

				$decls
			}
		);

		let implementation = quote_item!(&*context,
			impl $name {
				pub fn new() -> $name {
					$name {
						entities: ::std::collections::HashSet::new(),
						next_id : 0,
						$inits
					}
				}

				pub fn from_entities(entities: Vec<$entity_name>) -> $name {
					let mut world = $name {
						entities: ::std::collections::HashSet::new(),
						next_id : 0,
						$inits
					};

					for entity in entities.move_iter() {
						world.entities.insert(entity.id);
						if entity.id > world.next_id {
							world.next_id = entity.id + 1;
						}

						$imports
					}

					world
				}

				pub fn to_entities(&self) -> Vec<$entity_name> {
					self.entities
						.iter()
						.map(|id|
							$entity_name {
								id: *id,
								$entity_init
							})
						.collect()
				}

				$create_fns

				$import_fns

				pub fn import_entity(&mut self, entity: $entity_name) {
					let world = self;
					$imports
				}

				pub fn destroy_entity(&mut self, id: ::rustecs::EntityId) {
					self.entities.remove(&id);

					$removes
				}
			}
		);

		let entity_struct = quote_item!(&*context,
			#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
			pub struct $entity_name {
				pub id: ::rustecs::EntityId,
				$entity_decls
			}
		);

		let entity_impl = quote_item!(&*context,
			impl ::rustecs::Entity for $entity_name {
				fn id(&self) -> ::rustecs::EntityId {
					self.id
				}
			}
		);

		let mut items = Vec::new();
		items.push(structure.unwrap());
		items.push(implementation.unwrap());
		items.push(entity_struct.unwrap());
		items.push(entity_impl.unwrap());

		World(items)
	}

	fn entities(
		entities    : &Vec<ast::Ident>,
		all_entities: &Vec<Entity>
	) -> Vec<Entity> {
		all_entities
			.iter()
			.filter(|entity| {
				for &name in entities.iter() {
					if entity.name == name {
						return true;
					}
				}
				return false;
			})
			.map(|entity|
				entity.clone())
			.collect()
	}

	fn components(entities: &Vec<Entity>) -> HashMap<String, Component> {
		let mut components = HashMap::new();

		for entity in entities.iter() {
			for (name, component) in entity.components.iter() {
				components.insert((*name).clone(), (*component).clone());
			}
		}

		components
	}

	fn component_decls(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.decl.as_slice());
		}

		tokens
	}

	fn component_inits(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.init.as_slice());
		}

		tokens
	}

	fn imports(components: &HashMap<String, Component>) -> Vec<ast::TokenTree> {
		let mut tokens = Vec::new();

		for (_, component) in components.iter() {
			tokens.push_all(component.import.as_slice());
		}

		tokens
	}

	fn create_fns(entities: &Vec<Entity>) -> Vec<ast::TokenTree> {
		let mut tokens = Vec::new();

		for entity in entities.iter() {
			tokens.push_all(entity.create_fn.as_slice());
		}

		tokens
	}

	fn import_fns(entities: &Vec<Entity>) -> Vec<ast::TokenTree> {
		let mut tokens = Vec::new();

		for entity in entities.iter() {
			tokens.push_all(entity.import_fn.as_slice());
		}

		tokens
	}

	fn removes(components: &HashMap<String, Component>) -> Vec<ast::TokenTree> {
		let mut removes = Vec::new();

		for (_, component) in components.iter() {
			removes.push_all(component.remove.as_slice());
		}

		removes
	}

	fn entity_name(world_name: ast::Ident) -> ast::Ident {
		let name = token::get_ident(world_name)
			.to_str()
			.append("Entity".as_slice());

		ast::Ident::new(
			token::intern(name.as_slice()))
	}

	fn entity_decls(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut decls = Vec::new();

		for (_, component) in components.iter() {
			decls.push_all(component.entity_decl.as_slice());
		}

		decls
	}

	fn entity_init(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut init = Vec::new();

		for (_, component) in components.iter() {
			init.push_all(component.entity_init.as_slice());
		}

		init
	}
}


fn ident_to_lower(ident: ast::Ident) -> String {
	token::get_ident(ident)
		.to_str()
		.as_slice()
		.to_ascii_lower()
}
