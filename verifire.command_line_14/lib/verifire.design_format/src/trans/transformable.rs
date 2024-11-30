use std::f64::consts::PI;
use i_overlay::i_shape::int::path::IntPath;
use i_overlay::i_shape::int::shape::{IntShape, IntShapes};
use crate::fix::mark::Mark;
use crate::trans::reflection::Reflection;
use crate::trans::rotation::Rotation;
use crate::trans::scale::Scale;
use crate::trans::transform::Transform;
use crate::trans::translation::Translation;

pub(crate) trait Translate {
    fn transform(&mut self, transform: &Transform);
    fn reflect(&mut self, reflection: &Reflection);
    fn rotate(&mut self, rotation: &Rotation);
    fn scale(&mut self, scale: &Scale);
    fn translate(&mut self, translate: &Translation);
}

const TO_RADIAN: f64 = PI / 180.0;

impl Translate for IntPath {
    fn transform(&mut self, transform: &Transform) {
        self.reflect(&transform.reflection);
        self.rotate(&transform.rotation);
        self.scale(&transform.scale);
        self.translate(&transform.translate);
    }

    fn reflect(&mut self, reflection: &Reflection) {
        match reflection {
            Reflection::None => {}
            Reflection::Horizontal => {
                for p in self.iter_mut() {
                    p.y = -p.y;
                }
                // after reflection path order must be reversed
                self.reverse()
            }
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::None => {
                return;
            }
            Rotation::Deg90 => {
                for p in self.iter_mut() {
                    let (x, y) = (p.x, p.y);
                    p.x = -y;
                    p.y = x;
                }
            }
            Rotation::Deg180 => {
                for p in self.iter_mut() {
                    p.x = -p.x;
                    p.y = -p.y;
                }
            }
            Rotation::Deg270 => {
                for p in self.iter_mut() {
                    let (x, y) = (p.x, p.y);
                    p.x = y;
                    p.y = -x;
                }
            }
            Rotation::Deg(angle) => {
                let (sin, cos) = (angle * TO_RADIAN).sin_cos();
                for p in self.iter_mut() {
                    let x = p.x as f64;
                    let y = p.y as f64;
                    p.x = (x * cos - y * sin).round() as i32;
                    p.y = (x * sin + y * cos).round() as i32;
                }
            }
        }
    }

    fn scale(&mut self, scale: &Scale) {
        match scale {
            Scale::None => {}
            Scale::Int(scale) => {
                for p in self.iter_mut() {
                    p.x *= scale;
                    p.y *= scale;
                }
            }
            Scale::Float(scale) => {
                for p in self.iter_mut() {
                    let x = p.x as f64;
                    let y = p.y as f64;
                    p.x = (x * scale).round() as i32;
                    p.y = (y * scale).round() as i32;
                }
            }
        }
    }

    fn translate(&mut self, translate: &Translation) {
        match translate {
            Translation::None => {}
            Translation::Int(vec) => {
                for p in self.iter_mut() {
                    p.x += vec.x;
                    p.y += vec.y;
                }
            }
        }
    }
}

impl Translate for IntShape {
    fn transform(&mut self, transform: &Transform) {
        for path in self.iter_mut() {
            path.transform(transform);
        }
    }

    fn reflect(&mut self, reflection: &Reflection) {
        for path in self.iter_mut() {
            path.reflect(reflection);
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        for path in self.iter_mut() {
            path.rotate(rotation);
        }
    }

    fn scale(&mut self, scale: &Scale) {
        for path in self.iter_mut() {
            path.scale(scale);
        }
    }

    fn translate(&mut self, translate: &Translation) {
        for path in self.iter_mut() {
            path.translate(translate);
        }
    }
}

impl Translate for IntShapes {
    fn transform(&mut self, transform: &Transform) {
        for shape in self.iter_mut() {
            shape.transform(transform);
        }
    }

    fn reflect(&mut self, reflection: &Reflection) {
        for shape in self.iter_mut() {
            shape.reflect(reflection);
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        for shape in self.iter_mut() {
            shape.rotate(rotation);
        }
    }

    fn scale(&mut self, scale: &Scale) {
        for shape in self.iter_mut() {
            shape.scale(scale);
        }
    }

    fn translate(&mut self, translate: &Translation) {
        for shape in self.iter_mut() {
            shape.translate(translate);
        }
    }
}

impl Translate for Mark {
    fn transform(&mut self, transform: &Transform) {
        self.translate(&transform.translate);
    }

    fn reflect(&mut self, _: &Reflection) {}

    fn rotate(&mut self, _: &Rotation) {}

    fn scale(&mut self, _: &Scale) {}

    fn translate(&mut self, translate: &Translation) {
        match translate {
            Translation::None => {}
            Translation::Int(vec) => {
                self.point.x += vec.x;
                self.point.y += vec.y;
            }
        }
    }
}