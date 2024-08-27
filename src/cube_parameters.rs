use crate::constants::*;
use crate::cube_actions::CubeAction;

#[derive(Clone, Copy)]
pub struct CubeParameters {
  pub alpha: f32,
  pub beta: f32,
  pub gamma: f32,
  pub auto_alpha: f32,
  pub auto_beta: f32,
  pub auto_gamma: f32,
  pub distance_from_camera: f32,
  pub projection_scale: f32,
  pub resolution_step: f32,
}

impl CubeParameters {
  pub fn new() -> Self {
      Self {
          alpha: 0.0,
          beta: 0.0,
          gamma: 0.0,
          auto_alpha: 0.0,
          auto_beta: 0.0,
          auto_gamma: 0.0,
          distance_from_camera: DISTANCE_FROM_CAMERA,
          projection_scale: PROJECTION_SCALE,
          resolution_step: RESOLUTION_STEP,
      }
  }

  pub fn reset(&mut self) {
      *self = Self::new();
  }

  pub fn update_auto_rotations(&mut self) {
      self.alpha += 2.5 * self.auto_alpha;
      self.beta += 2.0 * self.auto_beta;
      self.gamma += 1.5 * self.auto_gamma;
  }

  pub fn apply_action(&mut self, action: CubeAction) {
      match action {
          CubeAction::ToggleAutoAlpha => self.auto_alpha = 1.0 - self.auto_alpha,
          CubeAction::IncreaseAlpha => self.alpha += 2.5,
          CubeAction::DecreaseAlpha => self.alpha -= 5.0,
          
          CubeAction::ToggleAutoBeta => self.auto_beta = 1.0 - self.auto_beta,
          CubeAction::IncreaseBeta => self.beta += 2.5,
          CubeAction::DecreaseBeta => self.beta -= 5.0,
          
          CubeAction::ToggleAutoGamma => self.auto_gamma = 1.0 - self.auto_gamma,
          CubeAction::IncreaseGamma => self.gamma += 2.5,
          CubeAction::DecreaseGamma => self.gamma -= 5.0,
          
          CubeAction::IncreaseDistanceFromCamera => self.distance_from_camera += 1.5,
          CubeAction::DecreaseDistanceFromCamera => self.distance_from_camera -= 1.0,
          
          CubeAction::IncreaseProjectionScale => self.projection_scale += 1.5,
          CubeAction::DecreaseProjectionScale => self.projection_scale -= 1.0,
          
          CubeAction::IncreaseResolutionStep => self.resolution_step += 0.1,
          CubeAction::DecreaseResolutionStep => self.resolution_step -= 0.1,
          
          CubeAction::Reset => self.reset(),
          CubeAction::Quit => std::process::exit(0),
      }
  }
}