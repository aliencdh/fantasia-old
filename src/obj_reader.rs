use crate::Vec3;
use color_eyre::eyre;
use std::str::FromStr;

pub type Vertex = Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Model {
    pub faces: Vec<Face>,
    pub vertices: Vec<Vertex>,
}
impl Model {
    pub fn empty() -> Self {
        Self {
            faces: vec![],
            vertices: vec![],
        }
    }
}
impl FromStr for Model {
    type Err = eyre::Error;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let mut rv = Model::empty();
        for line in src.split('\n') {
            if let Ok(vertex) = Vertex::from_str(line.trim()) {
                rv.vertices.push(vertex);
            } else if let Ok(face) = Face::from_str(line.trim()) {
                rv.faces.push(face);
            }
        }

        Ok(rv)
    }
}

impl FromStr for Vertex {
    type Err = eyre::Error;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        if let Some('v') = src.chars().next() {
        } else {
            eyre::bail!("Invalid string: {src}");
        }

        let data = src
            .split(' ')
            .skip(1)
            .flat_map(|s| f32::from_str(s.trim()).map_err(|err| eyre::eyre!("{err:?}")))
            .collect::<Vec<_>>();
        if data.len() != 3 {
            eyre::bail!("Invalid string: {src}");
        }

        Ok(Self {
            x: data[0],
            y: data[1],
            z: data[2],
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Face {
    pub indices: Vec<usize>,
}
impl FromStr for Face {
    type Err = eyre::Error;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        if let Some('f') = src.chars().next() {
        } else {
            eyre::bail!("Invalid string: {src}");
        }

        let data = src.split(' ').skip(1).collect::<Vec<_>>();
        let indices = data
            .iter()
            .flat_map(|s| {
                s.split('/')
                    .next()
                    .ok_or(eyre::eyre!("Invalid string: {src}"))
                    .and_then(|s| usize::from_str(s.trim()).map_err(|err| eyre::eyre!("{err:?}")))
            })
            .collect::<Vec<_>>();
        if indices.len() != 3 {
            eyre::bail!("Invalid string: {src}");
        }

        Ok(Self { indices })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_from_str() {
        let input = "f 1193/1240/1193 1180/1227/1180 1179/1226/1179";
        let expected = Face {
            indices: vec![1193, 1180, 1179],
        };
        let got = Face::from_str(input).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_vertex_from_str() {
        let input = "v 0.608654 -0.568839 -0.416318";
        let expected = Vertex::new(0.608654, -0.568839, -0.416318);
        let got = Vertex::from_str(input).unwrap();
        assert_eq!(expected, got);
    }
}
