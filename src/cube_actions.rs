use crate::cube_parameters::CubeParameters;
use crossterm::event::KeyCode;
use std::sync::mpsc;

pub enum CubeAction {
  ToggleAutoAlpha,
  IncreaseAlpha,
  DecreaseAlpha,
  
  ToggleAutoBeta,
  IncreaseBeta,
  DecreaseBeta,
  
  ToggleAutoGamma,
  IncreaseGamma,
  DecreaseGamma,
  
  IncreaseDistanceFromCamera,
  DecreaseDistanceFromCamera,
  
  IncreaseProjectionScale,
  DecreaseProjectionScale,

  IncreaseResolutionStep,
  DecreaseResolutionStep,
  
  Reset,
  Quit,
}

fn key_to_action(key: KeyCode) -> Option<CubeAction> {
  match key {
      KeyCode::Char('a') => Some(CubeAction::ToggleAutoAlpha),
      KeyCode::Char('e') => Some(CubeAction::IncreaseAlpha),
      KeyCode::Char('r') => Some(CubeAction::DecreaseAlpha),
      
      KeyCode::Char('b') => Some(CubeAction::ToggleAutoBeta),
      KeyCode::Char('d') => Some(CubeAction::IncreaseBeta),
      KeyCode::Char('f') => Some(CubeAction::DecreaseBeta),
      
      KeyCode::Char('g') => Some(CubeAction::ToggleAutoGamma),
      KeyCode::Char('c') => Some(CubeAction::IncreaseGamma),
      KeyCode::Char('v') => Some(CubeAction::DecreaseGamma),
      
      KeyCode::Char('u') => Some(CubeAction::IncreaseDistanceFromCamera),
      KeyCode::Char('j') => Some(CubeAction::DecreaseDistanceFromCamera),
      
      KeyCode::Char('i') => Some(CubeAction::IncreaseProjectionScale),
      KeyCode::Char('k') => Some(CubeAction::DecreaseProjectionScale),
      
      KeyCode::Char('o') => Some(CubeAction::IncreaseResolutionStep),
      KeyCode::Char('l') => Some(CubeAction::DecreaseResolutionStep),
      
      KeyCode::Char('z') => Some(CubeAction::Reset),
      KeyCode::Char('q') => Some(CubeAction::Quit),
      _ => None,
  }
}

pub fn process_input(rx: &mpsc::Receiver<KeyCode>, params: &mut CubeParameters) {
  if let Ok(key_code) = rx.try_recv() {
      if let Some(action) = key_to_action(key_code) {
          params.apply_action(action);
      }
  }
}
