#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

#[cfg(target_os = "emscripten")]
extern crate codewars2017_web;

extern crate codewars2017_log as game_log;

pub ( crate ) use game_log::GameLog;

mod camera;

pub ( crate ) use camera::*;

mod game_map;

use game_map::GameMap;

mod vehicles;

use vehicles::Vehicles;

mod blur;

mod skybox;

use skybox::SkyBox;

mod minimap;

use minimap::Minimap;

mod obj;

mod settings;

pub ( crate ) use settings::Settings;

mod effects;

use effects::Effects;

mod shadow_map;

use shadow_map::ShadowMap;

pub ( crate ) use shadow_map::ShadowCastMaterial;

mod material;

pub ( crate ) use material::*;

mod facilities;

use facilities::Facilities;

struct CodeWars2017 {
    app: Rc<codevisual::Application>,
    paused: Rc<Cell<bool>>,
    camera: Camera,
    skybox: SkyBox,
    map: GameMap,
    vehicles: Vehicles,
    facilities: Facilities,
    minimap: Minimap,
    effects: Effects,
    shadow_map: ShadowMap,
    game_log_loader: game_log::Loader,
    current_time: Rc<Cell<f32>>,
    settings: Rc<Settings>,
}

shader_library! {
    ShaderLib {
        "codewars" => include_str!("lib.glsl"),
        "camera" => include_str!("camera/lib.glsl"),
        "shadow" => include_str!("shadow_map/lib.glsl"),
    }
}

resources! {
    Resources {
        game_log_loader: game_log::Loader = "game.log",
        skybox: skybox::Resources = (),
        map: game_map::Resources = (),
        vehicles: vehicles::Resources = (),
        effects: effects::Resources = (),
        facilities: facilities::Resources = (),
    }
}

impl codevisual::Game for CodeWars2017 {
    type Resources = Resources;

    fn get_title() -> String {
        String::from("CodeWars 2017")
    }

    fn new(app: &Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        #[cfg(target_os = "emscripten")]
        codewars2017_web::init_overlay();

        let settings = Settings::new(app);

        let game_log_loader: game_log::Loader = resources.game_log_loader;
        let map = GameMap::new(app, resources.map, &settings, &game_log_loader.read());
        let vehicles = Vehicles::new(app, &settings, resources.vehicles, &game_log_loader);
        let camera = Camera::new(app, &settings, map.size);
        let current_time = Rc::new(Cell::new(0.0));
        let paused = Rc::new(Cell::new(false));
        let minimap = Minimap::new(app, &game_log_loader.read(), &settings);
        let effects = Effects::new(app, resources.effects, &settings, &game_log_loader);
        let shadow_map = ShadowMap::new(app, &settings);
        let skybox = SkyBox::new(app, resources.skybox, &settings);
        let facilities = Facilities::new(app, resources.facilities, &settings, &game_log_loader.read());

        #[cfg(target_os = "emscripten")]
        {
            let current_time = current_time.clone();
            let game_log_loader = game_log_loader.clone();
            codewars2017_web::set_timeline_callback(move |pos| {
                current_time.set(pos * game_log_loader.read().tick_count as f32 / 60.0);
            });
        }
        #[cfg(target_os = "emscripten")]
        codewars2017_web::init_play_pause_button(paused.clone());

        #[cfg(target_os = "emscripten")]
        {
            let game_log = game_log_loader.read();
            let (name1, name2) = game_log.players.get_names();
            codewars2017_web::set_names(name1, name2);
        }

        Self {
            app: app.clone(),
            paused,
            skybox,
            camera,
            game_log_loader,
            map,
            vehicles,
            facilities,
            effects,
            minimap,
            shadow_map,
            current_time,
            settings,
        }
    }

    fn update(&mut self, delta_time: f64) {
        if !self.paused.get() {
            let delta_time = delta_time * self.settings.time_scale.get();
            let new_time = f32::min(
                self.current_time.get() + delta_time as f32,
                (self.game_log_loader.read().loaded_tick_count - 1) as f32 / 60.0);
            self.current_time.set(f32::max(self.current_time.get(), new_time));
        }
        #[cfg(target_os = "emscripten")]
        codewars2017_web::set_playback_position((self.current_time.get() * 60.0) as usize,
                                                self.game_log_loader.read().tick_count);
    }

    fn draw(&mut self) {
        let tick_time = (self.current_time.get() * 60.0);
        let tick = tick_time as usize;
        let max_tick = self.game_log_loader.read().loaded_tick_count - 1;
        if !self.paused.get() && tick <= max_tick {
            let (score1, score2) = self.game_log_loader.read().players.get_scores(tick);
            #[cfg(target_os = "emscripten")]
            codewars2017_web::set_scores(score1, score2);

            let mut framebuffer = self.app.ugli_context().default_framebuffer();
            let framebuffer = &mut framebuffer;
            let uniforms = (
                uniforms! {
                    u_sky_height: self.settings.sky_height.get() as f32,
                    u_current_time: self.current_time.get() as f32,
                    u_cell_size: 32.0, // TODO
                },
                self.camera.uniforms());
            //            self.skybox.draw(framebuffer, &uniforms);
            //            ugli::clear(framebuffer, None, Some(1.0));
            ugli::clear(framebuffer, Some(Color::BLACK), Some(1.0));

            self.vehicles.update_to(tick_time);

            let uniforms = (&uniforms, if self.settings.shadows_enabled.get() {
                let mut shadow_map = self.shadow_map.get_framebuffer(framebuffer.get_size());
                {
                    let framebuffer: &mut ugli::Framebuffer = &mut shadow_map;
                    self.vehicles.draw_shadows(framebuffer, &uniforms);
                    self.map.trees.draw_shadows(framebuffer, &uniforms);
                    self.facilities.draw_shadows(framebuffer, &uniforms);
                }
                Some(ugli::SingleUniform::new(
                    "u_shadow_map",
                    shadow_map.get_texture()))
            } else { None });

            self.map.ground.draw(framebuffer, &uniforms);
            self.map.trees.draw(framebuffer, &uniforms);
            self.vehicles.draw(framebuffer, &uniforms);
            self.facilities.draw(framebuffer, &uniforms);
            self.effects.draw(framebuffer, &uniforms, tick);
            self.map.weather.draw(framebuffer, &uniforms);
            if self.settings.draw_minimap.get() {
                self.minimap.draw(&self.vehicles, &self.map, &self.camera, framebuffer, &uniforms);
            }
        }
    }

    fn handle_event(&mut self, event: codevisual::Event) {
        self.camera.handle(event);
    }
}

fn main() {
    #[cfg(target_os = "emscripten")]
    codewars2017_web::init();

    #[cfg(not(target_os = "emscripten"))]
    std::env::set_current_dir("static").unwrap();
    codevisual::run::<CodeWars2017>()
}