use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::CANVAS_W;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlAudioElement};

pub struct ScoreEntity{
    font_name: &'static str,
    hi_str: String,
    score_str: String,
    remaining_ds: u8,
    save_elapsed: u16,
    blink_count: u16,
    audio: HtmlAudioElement
}

impl ScoreEntity{
    pub fn new(ft_name: &'static str, audio: &HtmlAudioElement) -> Self{
        ScoreEntity{
            font_name: ft_name,
            hi_str: "".to_string(),
            score_str: "00000".to_string(),
            remaining_ds: 0,
            save_elapsed: 0,
            blink_count: 2000,
            audio: audio.clone()
        }
    }

    fn format_score(score: &u64) -> String{
        if *score >= 100000{
            format!("{:.2}M", *score as f64 / 1000000.0)
        } else {
            format!("{:05}", score)
        }
    }

    fn process_save_hiscore(&mut self, game_data: &mut GameData){
        if self.save_elapsed > 1000 {
            self.save_elapsed = 0;
            game_data.save_hiscore().unwrap();
        }
    }

    fn process_blink(&mut self, delta_ms: u16, should_blink: bool){
        if should_blink {
            self.blink_count = 0;
            let _ = self.audio.play();
        }
        if self.blink_count > 2000{
            self.blink_count = 2000;
            return;
        }
        self.blink_count += delta_ms;
    }

    fn process_hiscore(&mut self, game_data: &mut GameData){
        let must_update = game_data.hiscore < game_data.score;
        if must_update { game_data.hiscore = game_data.score; }

        if must_update || self.hi_str.len() == 0 {
            self.hi_str = format!("HI {}", Self::format_score(&game_data.hiscore));
        }
    }

    fn process_score(&mut self, delta_ms: u16, data: &mut GameData) -> u64 {
        let plus_score_16 = (data.speed * delta_ms as f64) as u16 + self.remaining_ds as u16 ;
        self.remaining_ds = (plus_score_16 % 60) as u8;
        let plus_score_8 = plus_score_16 as u64/ 60;
        data.score += plus_score_8;
        self.score_str = Self::format_score(&data.score);
        plus_score_8

    }
}

impl ScoreEntity{
    fn init_draw(&self, ctx: &CanvasRenderingContext2d){
        ctx.set_font(self.font_name)
    }
}

impl StaticEntity<GameData> for ScoreEntity {}

impl Drawable for ScoreEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.init_draw(ctx);
        ctx.set_fill_style(&"#DADADA".into());
        ctx.fill_text(&self.hi_str, CANVAS_W - 160.0, 15.0)?;
        if self.blink_count % 1000 < 500 {
            ctx.set_fill_style(&"#535353".into());
            ctx.fill_text(&self.score_str, CANVAS_W - 55.0, 15.0)?;
        }
        Ok(())
    }
}

impl Processable<GameData> for ScoreEntity {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        self.process_hiscore(data);
        self.save_elapsed += delta_ms;
        self.process_save_hiscore(data);

        if data.game_over { data.score = 0;}
        if data.pause { return Ok(()); }

        let plus_score = self.process_score(delta_ms, data);
        let should_blink = (data.score > 99) && (data.score % 100 < plus_score);
        self.process_blink(delta_ms,  should_blink);

        Ok(())
    }
}

impl EventListener for ScoreEntity {}

impl EngineEntity<GameData> for ScoreEntity {}