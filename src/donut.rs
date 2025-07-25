use std::f32::consts::PI;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::{Line, Span, Text}, widgets::{Block, Widget}};
use crate::Rat;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const DPI: f32 = 2.*PI;

pub struct Donut {
    theta_spacing: f32,
    phi_spacing: f32,
    r1: f32,
    r2: f32,
    k1: f32,
    k2: f32
}

impl Default for Donut {
    fn default() -> Self {
        let (
            theta_spacing,
            phi_spacing,
            r1,
            r2,
            k2
        ) = (
            0.07,
            0.02,
            1.,
            2.,
            5.
        );

        Donut {
            theta_spacing,
            phi_spacing,
            r1,
            r2,
            k1: WIDTH as f32 * k2 * 3. / (8. * ( r1 + r2 )),
            k2,
        }
    }
}

impl Donut {
    fn compute_donut(&self, a: f32, b: f32) -> [[char; HEIGHT]; WIDTH] {
        let (
            cosa,
            sina,
            cosb,
            sinb
        ) = (
            a.cos(),
            a.sin(),
            b.cos(),
            b.sin()
        );

        let (mut output, mut zbuffer) = (
            [[' '; HEIGHT]; WIDTH],
            [[0.; HEIGHT]; WIDTH]
        );

        let (mut theta, mut phi) = (0., 0.);
        while theta < DPI {
            let (costheta, sintheta) = (theta.cos(), theta.sin());

            while phi < DPI {
                let (cosphi, sinphi) = (phi.cos(), phi.sin());
                let (circlex, circley) = (self.r2 + self.r1 * costheta, self.r1 * sintheta);
                let (
                    x,
                    y,
                    z
                ) = (
                    circlex * (cosb * cosphi + sina * sinb * sinphi) - circley * cosa * sinb,
                    circlex * (sinb * cosphi - sina * cosb * sinphi) + circley * cosa * cosb,
                    self.k2 + cosa * circlex * sinphi + circley * sina,
                );
                let rz = 1./z;
                let (xp, yp) = (
                    WIDTH / 2 + (self.k1 * rz * x) as usize,
                    HEIGHT / 2 - (self.k1 * rz * y) as usize
                );
                let luminance =
                    cosphi * costheta * sinb -
                    cosa * costheta * sinphi -
                    sina * sintheta +
                    cosb * (cosa * sintheta - costheta * sina * sinphi)
                ;

                if luminance > 0. {
                    if rz > zbuffer[xp][yp] {
                        zbuffer[xp][yp] = rz;
                        let luminance_index = luminance as usize * 8;
                        output[xp][yp] = b".,-~:;=!*#$@"[luminance_index] as char;
                    }
                }

                phi += self.phi_spacing;
            }
            theta += self.theta_spacing;
        }
        output
    }
}

impl Rat {
    pub fn handle_donut(&mut self, kv: KeyEvent) {
        match kv.code {
            KeyCode::Up => self.donut.k2 += 1.,
            KeyCode::Down => self.donut.k2 -= 1.,
            _ => {}
        }
    }

    pub fn render_donut(&self, area: Rect, buf: &mut Buffer) {
        self.donut.compute_donut(0.,0.)
            .iter().map(|x| Line::from(x.iter().collect::<String>()))
            .collect::<Text>()
            .render(area, buf)
        ;
        Block::bordered().title_top(Span::from("A simple donut, but it's not working...").italic().into_centered_line()).blue().render(area, buf);
    }
}

