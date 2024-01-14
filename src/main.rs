#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(lazy_cell)]
#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::type_complexity)]
#![allow(clippy::borrow_interior_mutable_const)]

use std::cell::LazyCell;
use eframe::egui::{self};
use egui::Color32;
use either::Either;
use serde::{Deserialize, Serialize};
use core::cmp::Ordering;
use std::fs;
fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum Piece2 {
    WK,
    WQ,
    WR,
    WB,
    WN,
    WP,
    BK,
    BQ,
    BR,
    BB,
    BN,
    BP,
}

impl Piece2{
    fn est_blanche(&self) -> bool {
        self.clone() as u32 <= 5 
    }
    fn est_differente(&self, autre:&Self) -> bool {
        self.est_blanche() != autre.est_blanche()
    }
    fn dessine(self) -> Box<str> {
       char::from_u32('♔' as u32 + self as u32)
                .unwrap()
                .to_string()
                .into()
    }
}


#[derive(Default)]
struct MyEguiApp {
    board: Vec<Vec<Option<Piece2>>>,
    je_suis_noir: bool,
    tour_de_noir: bool,
    est_selectionné: Option<((usize, usize), Vec<(usize, usize)>)>,
    hovered: Option<(usize, usize)>,
    pointer_pos: Option<egui::Pos2>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            board: DEPART.clone(),
            ..Default::default()
        }
    }
    fn  mouvements_possible(&self, piece: &Piece2, x:usize, y:usize) -> Vec<(usize, usize)> {
        use Piece2::*;
        fn cond(d: isize, val:usize) -> bool{
            match d.cmp(&0) {
                Ordering::Greater => val < 8_usize.checked_add_signed(-d).unwrap(),
                Ordering::Less => val > (-1_isize-d) as usize,
                Ordering::Equal => true
            }
        }
        let saut = |v: &mut Vec<(usize, usize)>, dx: isize, dy: isize| -> bool{
            if cond(dx, x) && cond(dy, y){
                let x = x.checked_add_signed(dx).unwrap();
                let y = y.checked_add_signed(dy).unwrap();
                v.push((x, y));
                true
            } else {
                false
            }
        };
        let ajoute_ligne = |v: &mut Vec<(usize, usize)>, dx: isize, dy: isize|{
            let mut x =x;
            let mut y =y;
            while cond(dx, x) && cond(dy, y) {
                x = x.checked_add_signed(dx).unwrap();
                y = y.checked_add_signed(dy).unwrap();
                if let Some(autre) = &self.board[x][y] {
                    if piece.est_differente(autre){
                        v.push((x, y));
                    }
                    break;
                }
                v.push((x, y));
            }
        };
        let coups = match piece{
            WK | BK => {
                let mut v = vec![];
                saut(&mut v, 1, 1);
                saut(&mut v, 1, -1);
                saut(&mut v, 1, 0);
                saut(&mut v, 0, 1);
                saut(&mut v, 0, 0);
                saut(&mut v, 0, -1);
                saut(&mut v, -1, 1);
                saut(&mut v, -1, -1);
                saut(&mut v, -1, 0);

                
                v
            },
            WR |BR => {
                let mut v = vec![];
                ajoute_ligne(&mut v, 1, 0);
                ajoute_ligne(&mut v, 1, 0);
                ajoute_ligne(&mut v, 0, 1);
                ajoute_ligne(&mut v, 0, -1);
                ajoute_ligne(&mut v, -1, 0);
                return v;
            }

            WB | BB => {
                let mut v = vec![];
                ajoute_ligne(&mut v, 1, 1);
                ajoute_ligne(&mut v, 1, -1);
                ajoute_ligne(&mut v, -1, 1);
                ajoute_ligne(&mut v, -1, -1);
                return v;
            }

            WN | BN => {
                let mut v = vec![];
                saut(&mut v, 2, 1);
                saut(&mut v, 2, -1);
                saut(&mut v, 1, 2);
                saut(&mut v, 1, -2);
                saut(&mut v, -1, 2);
                saut(&mut v, -1, -2);
                saut(&mut v, -2, 1);
                saut(&mut v, -2, -1);    
                v
            }

            WQ | BQ => {
                let mut v = vec![];
                ajoute_ligne(&mut v, 1, 1);
                ajoute_ligne(&mut v, 1, 0);
                ajoute_ligne(&mut v, 1, -1);
                ajoute_ligne(&mut v, 0, 1);
                ajoute_ligne(&mut v, 0, -1);
                ajoute_ligne(&mut v, -1, 1);
                ajoute_ligne(&mut v, -1, 0);
                ajoute_ligne(&mut v, -1, -1);
                return v;
            }
            BP => {
                let mut v = vec![];
                if x==6 && self.board[5][y].is_none() && self.board[4][y].is_none(){
                    v.push((4, y))
                }
                if cond(-1, x) && self.board[x-1][y].is_none(){
                    v.push((x-1, y))
                }
                if cond(-1, x) && cond(1, y) && self.board[x-1][y+1].is_some(){
                    v.push((x-1, y+1))
                }
                if cond(-1, x) && cond(-1, y) && self.board[x-1][y-1].is_some(){
                    v.push((x-1, y-1))
                }
                v
            }
            WP => {
                let mut v = vec![];
                if x==1 && self.board[2][y].is_none() && self.board[3][y].is_none(){
                    v.push((3, y))
                }
                if cond(-1, x) && self.board[x+1][y].is_none(){
                    v.push((x+1, y))
                }
                if cond(-1, x) && cond(1, y) && self.board[x+1][y+1].is_some(){
                    v.push((x+1, y+1))
                }
                if cond(-1, x) && cond(-1, y) && self.board[x+1][y-1].is_some(){
                    v.push((x+1, y-1))
                }
                v
            }
        };
        println!("{coups:?}");
        let pblanc = piece.est_blanche();

       /* let mut v2 =vec![];
        for coup in coups.into_iter() {
            if let Some (piece2) = &self.board[coup.0][coup.1] {
                if pblanc == piece2.est_blanche() {
                    continue;
                }
            }
            v2.push(coup);
        }
        v2 */

        coups.into_iter().filter(|coup| 
            self.board[coup.0][coup.1].clone()
                .map(|p| p.est_blanche()) != Some(pblanc)
        ).collect()
    }
}
const DEPART: LazyCell<Vec<Vec<Option<Piece2>>>> = LazyCell::new(||ron::from_str(include_str!("../board2.ron")).unwrap());

type X2 = Option<Piece2>;
fn dessine_piece2(case: X2) -> Box<str> {
    match case {
        Some(piece) => piece.dessine(),
        None => "    ".into(),
    }
}

fn dessine_case(case: X2, background: egui::Color32) -> egui::RichText {
    egui::RichText::new(dessine_piece2(case))
        .size(60.0)
        .background_color(background)
        .color(Color32::BLACK)
}/* Rouge : #c0584a */
const MON_BLEU: Color32 = Color32::from_rgb(0x4b, 0x73, 0x99);
impl eframe::App for MyEguiApp {
    /// This function will be called each time the UI needs repainting.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::right("my_right_panel").resizable(false).show(ctx, |ui| {
            ui.label("Hello World!");
            if ui.toggle_value(&mut self.je_suis_noir, "noir").clicked() {
                println!("A Cliqué sur blanc");
            }
            {
                let mut visuals = ui.ctx().style().visuals.clone();
                visuals.light_dark_radio_buttons(ui);
                ui.ctx().set_visuals(visuals);
            }
            if ui.button("Reset").clicked() {
                self.board = DEPART.clone();
                self.est_selectionné=None;
                self.tour_de_noir = false;
            }
            if ui.button("Decocher").clicked() {
                self.est_selectionné=None;
            }
            if ui.button("Sauvegarder").clicked() { 
                //let board2:Vec<Vec<Option<Piece2>>>= self.board.iter().map(|l| l.iter().map(|c| c.as_ref().map(|p| unsafe {std::mem::transmute::<u32, Piece2>(p.0 as u32 *6+p.1.clone() as u32)} )).collect()).collect();
                let r = ron::ser::to_string_pretty(
                    &self.board,
                    ron::ser::PrettyConfig::default().depth_limit(1),
                )
                .unwrap();
                fs::write("board2.ron", r).unwrap();
            }
        });
        let mut case_ou_bouger: Option<(usize, usize)> =None;
        egui::CentralPanel::default().show(ctx, |ui| {
        {

            let iteur = if self.je_suis_noir {
                Either::Left(self.board.clone().into_iter().enumerate())
            } else {
                Either::Right(self.board.clone().into_iter().enumerate().rev())
            };
            let mut est_white = false;
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                for (x, ligne) in iteur {
                    for (y , case) in ligne.iter().enumerate() {
                        let txt = dessine_case(
                            case.clone(),
                            if self.est_selectionné.is_some() && self.est_selectionné.as_ref().unwrap().0 == (x, y) { Color32::YELLOW } 
                                else if self.est_selectionné.is_some() && self.est_selectionné.as_ref().unwrap().1.iter().any(|pos| pos == &(x, y)) { Color32::GREEN } 
                                else if est_white { Color32::WHITE } 
                                else { MON_BLEU },
                        );
                        let response = ui.add(
                            egui::Button::new(txt).sense(egui::Sense::click_and_drag())
                        );
                        if response.clicked(){
                            println!("A Cliqué sur {case:?}");
                            if let Some(val) = &self.est_selectionné{
                                case_ou_bouger= Some((x, y));
                            } else if let Some(piece) = case{
                                if piece.est_blanche() != self.tour_de_noir{
                                    self.est_selectionné=Some(((x, y), self.mouvements_possible(piece, x, y)));
                                }
                            }
                        }
                        if response.drag_started(){
                            if let Some(piece) = case{
                                if piece.est_blanche() != self.tour_de_noir{
                                    self.est_selectionné=Some(((x, y), self.mouvements_possible(piece, x, y)));
                                } else{
                                    ui.memory_mut(|m| m.stop_dragging());
                                    self.pointer_pos = None;
                                }
                            }
                        }
                        if response.drag_released() {
                            if let Some(val) = &self.est_selectionné{
                                case_ou_bouger = self.hovered;
                                println!("A déposé {case:?} sur {case_ou_bouger:?}");
                                self.pointer_pos = None;
                            }
                        }
                        if response.dragged() {
                            self.pointer_pos = ui.ctx().input(|i| i.pointer.latest_pos());
                        }
                        if let Some(p) = self.pointer_pos{
                            if response.rect.contains(p){
                                if self.hovered != Some((x, y)){
                                    self.hovered = Some((x, y));
                                    println!("sur {:?}", (x, y));
                                }
                            }
                        }
                        
                        est_white = !est_white;
                    }
                    est_white = !est_white;
                    ui.end_row();
                }
            
                
            });
     } 
     if let Some(case) = case_ou_bouger{
        let piece =self.est_selectionné.as_ref().unwrap();
        if piece.1.iter().any(|pos| pos == &(case.0, case.1)){
            self.board[case.0][case.1] = self.board[piece.0.0][piece.0.1].take();
            self.est_selectionné = None;
            self.tour_de_noir = !self.tour_de_noir;
        }
    }
    });
    }
}

