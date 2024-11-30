use foundation::i_overlay::core::fill_rule::FillRule;
use foundation::i_overlay::core::overlay::{Overlay, ShapeType};
use foundation::i_overlay::core::overlay_rule::OverlayRule;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::ProgressNotificator;
use serde::{Deserialize, Serialize};
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::shape::IntShape;
use crate::solution::rail::{BitSegment, Rail};
use crate::solution::rail_builder::RailBuilder;
use crate::solution::sampling::Sampling;

struct DualIndex {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DensityResultType {
    #[serde(rename = "shapes")]
    Shapes,
    #[serde(rename = "rects")]
    Rects,
    #[serde(rename = "slices")]
    Slices,
}

pub struct DensitySolver {
    shapes_res: ShapeRes,
    density: f64,
    sampling_level: usize,
    window_width: i32,
    window_height: i32,
    result_type: DensityResultType,
}

impl DensitySolver {
    pub fn new(shapes_res: ShapeRes, density: f64, sampling_level: usize, window_width: i32, window_height: i32, result_type: DensityResultType) -> Result<Self, String> {
        if sampling_level < 6 || sampling_level > 16 {
            Err("sampling_level must be in range N=6...16 (2^N)".to_string())
        } else {
            Ok(Self { shapes_res, density, sampling_level, window_width, window_height, result_type })
        }
    }

    pub async fn find_problems<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> Vec<IntShape> {
        let shapes = self.shapes_res.shapes();
        let sampling = Sampling::new(&shapes, self.sampling_level);
        let rails = RailBuilder::build(shapes, &sampling);

        let iw = if sampling.x_metric.log2 == 0 {
            self.window_width as usize
        } else {
            let s = sampling.x_metric.step() as usize;
            self.window_width as usize / s
        }.max(1);

        let ih = if sampling.y_metric.log2 == 0 {
            self.window_height as usize
        } else {
            let s = sampling.y_metric.step() as usize;
            self.window_height as usize / s
        }.max(1);

        let max_mass = iw * ih;
        let min_mass = (self.density * (max_mass as f64)).round() as usize;

        let xl = if iw <= sampling.x_metric.count { sampling.x_metric.count - iw } else { 0 };
        let yl = if ih <= sampling.y_metric.count { sampling.y_metric.count - ih } else { 0 };

        let mut lines = vec![Rail::empty(); sampling.x_metric.count];
        let mut sub_lines = vec![Rail::empty(); iw];

        let mut scanners = Vec::with_capacity(iw);

        let mut old_progress = 0;
        let progress_scale = 1.0 / xl as f64;

        let mut windows = Vec::new();
        for x0 in 0..xl {
            let progress = (x0 as f64) * progress_scale;
            let new_progress = (20.0 * progress) as usize;
            if new_progress != old_progress {
                notificator.send(progress).await;
                old_progress = new_progress;
            }

            scanners.clear();

            let mut mass = 0;

            // init scanners
            for x in 0..iw {
                sub_lines[x].bit_segments.clear();

                let index = x0 + x;
                let rail = &rails[index];
                let scanner = rail.start(ih as u16);
                mass += scanner.sum as usize;
                scanners.push(scanner);
            }

            // iterate
            for y in 0..=yl {
                if mass < min_mass {
                    if self.result_type == DensityResultType::Rects {
                        windows.push(DualIndex { x: x0 as u16, y: y as u16 });
                    } else {
                        for x in 0..iw {
                            let new_seg = BitSegment { start: y as u16, end: (y + ih - 1) as u16 };
                            sub_lines[x].append(new_seg);
                        }
                    }
                }

                if y != yl {
                    mass = 0;
                    for x in 0..iw {
                        let index = x0 + x;
                        let rail = &rails[index];
                        mass += scanners[x].iterate(rail);
                    }
                }
            }

            if self.result_type != DensityResultType::Rects {
                // merge result into main
                for x in 0..iw {
                    let index = x0 + x;
                    lines[index].merge(&sub_lines[x]);
                }
            }
        }

        let dx = sampling.x_metric.step();
        let dy = sampling.y_metric.step();
        let ox = sampling.origin.x;
        let oy = sampling.origin.y;
        let oy0 = oy;
        let oy1 = oy + dy;// + (dy >> 1);

        match self.result_type {
            DensityResultType::Rects => {
                let mut rects = Vec::with_capacity(windows.len());
                let w = (iw as i32) * dx;
                let h = (ih as i32) * dy;

                for window in windows.iter() {
                    let x0 = ox + dx * (window.x as i32);
                    let y0 = oy + dy * (window.y as i32);
                    let x1 = x0 + w;
                    let y1 = y0 + h;

                    rects.push(vec![vec![
                        IntPoint::new(x0, y0),
                        IntPoint::new(x0, y1),
                        IntPoint::new(x1, y1),
                        IntPoint::new(x1, y0),
                    ]]);
                }

                rects
            }
            DensityResultType::Shapes | DensityResultType::Slices => {
                let mut slices = Vec::with_capacity(lines.len());

                // convert lines to rectangles
                for i in 0..sampling.x_metric.count {
                    let line = &lines[i];
                    if !line.is_empty() {
                        let x0 = ox + (i as i32) * dx;
                        let x1 = x0 + dx;
                        for s in line.bit_segments.iter() {
                            let y0 = (s.start as i32) * dy + oy0;
                            let y1 = (s.end as i32) * dy + oy1;

                            let rect = [
                                IntPoint::new(x0, y1),
                                IntPoint::new(x0, y0),
                                IntPoint::new(x1, y0),
                                IntPoint::new(x1, y1)
                            ];

                            slices.push(rect.to_vec());
                        }
                    }
                }

                if self.result_type == DensityResultType::Slices {
                    vec![slices]
                } else {
                    let mut overlay = Overlay::new(4 * slices.len());
                    overlay.add_paths(&slices, ShapeType::Subject);
                    overlay.into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject)
                }
            }
        }
    }

    pub fn debug_find_problems(&self) -> Vec<[IntPoint; 2]> {
        let shapes = self.shapes_res.shapes();
        let sampling = Sampling::new(&shapes, self.sampling_level);
        let rails = RailBuilder::build(shapes, &sampling);

        let mut result = Vec::with_capacity(8 * rails.len());
        let mut x = sampling.origin.x + sampling.x_metric.offset();
        let oy = sampling.origin.y;
        let dx = sampling.x_metric.step();
        let sy = sampling.y_metric.log2;

        for rail in rails.iter() {
            if !rail.is_empty() {
                for bit_seg in rail.bit_segments.iter() {
                    let y0 = oy + ((bit_seg.start as i32) << sy);
                    let y1 = oy + (((bit_seg.end + 1) as i32) << sy);
                    result.push([IntPoint { x, y: y0 }, IntPoint { x, y: y1 }]);
                }
            }
            x += dx;
        }

        result
    }
}