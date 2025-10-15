use crate::day::{DayPart, DaySolver};
use crate::parsers::single_input_line;
use anyhow::Context;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

type Pixel = u8;
const TRANSPARENT: Pixel = 2;

struct Layer<const WIDTH: usize, const HEIGHT: usize>([[Pixel; WIDTH]; HEIGHT]);

impl<const WIDTH: usize, const HEIGHT: usize> Layer<WIDTH, HEIGHT> {
    fn row_from_chunk(chunk: impl IntoIterator<Item = Pixel>) -> anyhow::Result<[Pixel; WIDTH]> {
        let row = chunk
            .into_iter()
            .collect_vec()
            .try_into()
            .map_err(|row| anyhow::anyhow!("Invalid row: {row:?}"))?;
        Ok(row)
    }

    fn parse_layer(pixel_iter: &mut impl Iterator<Item = Pixel>) -> anyhow::Result<Self> {
        let rows: Vec<[Pixel; WIDTH]> = pixel_iter
            .take(WIDTH * HEIGHT)
            .chunks(WIDTH)
            .into_iter()
            .map(Self::row_from_chunk)
            .try_collect()?;

        let layer: [[Pixel; WIDTH]; HEIGHT] = rows
            .try_into()
            .map_err(|rows| anyhow::anyhow!("Invalid layer: {rows:?}"))?;

        Ok(Self(layer))
    }

    fn as_pixel_counter(&self) -> HashMap<Pixel, usize> {
        self.0.iter().flat_map(|row| row.iter()).copied().counts()
    }

    fn combine_rows(front_row: &[Pixel; WIDTH], back_row: &[Pixel; WIDTH]) -> [Pixel; WIDTH] {
        front_row
            .iter()
            .copied()
            .zip(back_row.iter().copied())
            .map(|(front_pixel, back_pixel)| {
                if front_pixel == TRANSPARENT {
                    back_pixel
                } else {
                    front_pixel
                }
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn render(foreground: &Self, background: &Self) -> Self {
        let layer: [[Pixel; WIDTH]; HEIGHT] = foreground
            .0
            .iter()
            .zip(background.0.iter())
            .map(|(front_row, back_row)| Self::combine_rows(front_row, back_row))
            .collect_vec()
            .try_into()
            .unwrap();

        Self(layer)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for Layer<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.0.iter().enumerate() {
            if idx > 0 {
                writeln!(f)?;
            }

            for pixel in row {
                write!(f, "{}", pixel)?;
            }
        }

        Ok(())
    }
}

struct Image<const WIDTH: usize, const HEIGHT: usize> {
    layers: Vec<Layer<WIDTH, HEIGHT>>,
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Image<WIDTH, HEIGHT> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Vec<Pixel> = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|digit| digit as Pixel)
                    .ok_or_else(|| anyhow::anyhow!("Invalid digit: {c}"))
            })
            .try_collect()?;

        let mut digit_iter = digits.into_iter().peekable();
        let layers: Vec<_> = std::iter::from_fn(|| {
            if digit_iter.peek().is_some() {
                Some(Layer::parse_layer(&mut digit_iter.by_ref()))
            } else {
                None
            }
        })
        .try_collect()?;

        Ok(Self { layers })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for Image<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for layer in &self.layers {
            writeln!(f, "{}", layer)?;
        }

        Ok(())
    }
}

pub(crate) struct Day8 {}

impl Day8 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn parse_input<const WIDTH: usize, const HEIGHT: usize>(
        lines: &[&str],
    ) -> anyhow::Result<Image<WIDTH, HEIGHT>> {
        let line = single_input_line(lines)?;

        Image::from_str(line)
    }
}

impl DaySolver for Day8 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, anyhow::Error> {
        const WIDTH: usize = 25;
        const HEIGHT: usize = 6;
        let image: Image<WIDTH, HEIGHT> = Self::parse_input(input)?;

        match part {
            DayPart::Part1 => {
                let (_layer, pixel_counter) = image
                    .layers
                    .iter()
                    .map(|layer| (layer, layer.as_pixel_counter()))
                    .min_by_key(|(_layer, pixel_counter)| {
                        pixel_counter.get(&0).copied().unwrap_or_default()
                    })
                    .with_context(|| "No layers")?;

                let solution = pixel_counter.get(&1).copied().unwrap_or_default()
                    * pixel_counter.get(&2).copied().unwrap_or_default();

                Ok(Box::new(solution))
            }
            DayPart::Part2 => {
                let render = image
                    .layers
                    .into_iter()
                    .reduce(|front, back| Layer::render(&front, &back))
                    .expect("At least one layer");

                Ok(Box::new(render))
            }
        }
    }
}
