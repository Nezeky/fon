//! Speaker/channel positions within a speaker configuration.

use core::ops::{Index, IndexMut};

use crate::{chan::Channel, frame::Frame};

/// All directions
///  - Mono
#[derive(Copy, Clone, Debug)]
pub struct Mono;

/// Side Left (90 degrees left)
///  - Stereo
///  - 3.0
///  - 6.1
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct Left;

/// Side Right (90 degrees right)
///  - Stereo
///  - 3.0
///  - 6.1
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct Right;

/// Center (0/180 degrees left/right)
///  - 3.0
#[derive(Copy, Clone, Debug)]
pub struct Center;

/// Front Center (0 degrees left/right)
///  - 5.0
///  - 5.1
///  - 6.1
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct Front;

/// Front Left (30 degrees left)
///  - 3.0
///  - 4.0
///  - 5.0
///  - 5.1
///  - 6.1
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct FrontL;

/// Front Right (30 degrees right)
///  - 3.0
///  - 4.0
///  - 5.0
///  - 5.1
///  - 6.1
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct FrontR;

/// Left Surround (110 degrees left)
///  - 4.0
///  - 5.0
///  - 5.1
#[derive(Copy, Clone, Debug)]
pub struct SurroundL;

/// Right Surround (110 degrees right)
///  - 4.0
///  - 5.0
///  - 5.1
#[derive(Copy, Clone, Debug)]
pub struct SurroundR;

/// Low frequency effects (unimportant direction)
///  - 5.1
///  - 6.1
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct Lfe;

/// Back (180 degrees left/right)
///  - 6.1
#[derive(Copy, Clone, Debug)]
pub struct Back;

/// Back Left (150 degrees left)
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct BackL;

/// Back Right (150 degrees right)
///  - 7.1
#[derive(Copy, Clone, Debug)]
pub struct BackR;

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<Mono> for Frame<Chan, 1> {
    type Output = Chan;

    fn index(&self, _: Mono) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<Mono> for Frame<Chan, 1> {
    fn index_mut(&mut self, _: Mono) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<Left> for Frame<Chan, 2> {
    type Output = Chan;

    fn index(&self, _: Left) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<Left> for Frame<Chan, 2> {
    fn index_mut(&mut self, _: Left) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

impl<Chan: Channel> Index<Right> for Frame<Chan, 2> {
    type Output = Chan;

    fn index(&self, _: Right) -> &Self::Output {
        &self.channels()[1]
    }
}

impl<Chan: Channel> IndexMut<Right> for Frame<Chan, 2> {
    fn index_mut(&mut self, _: Right) -> &mut Self::Output {
        &mut self.channels_mut()[1]
    }
}

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<Left> for Frame<Chan, 3> {
    type Output = Chan;

    fn index(&self, _: Left) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<Left> for Frame<Chan, 3> {
    fn index_mut(&mut self, _: Left) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

impl<Chan: Channel> Index<Right> for Frame<Chan, 3> {
    type Output = Chan;

    fn index(&self, _: Right) -> &Self::Output {
        &self.channels()[1]
    }
}

impl<Chan: Channel> IndexMut<Right> for Frame<Chan, 3> {
    fn index_mut(&mut self, _: Right) -> &mut Self::Output {
        &mut self.channels_mut()[1]
    }
}

impl<Chan: Channel> Index<Center> for Frame<Chan, 3> {
    type Output = Chan;

    fn index(&self, _: Center) -> &Self::Output {
        &self.channels()[2]
    }
}

impl<Chan: Channel> IndexMut<Center> for Frame<Chan, 3> {
    fn index_mut(&mut self, _: Center) -> &mut Self::Output {
        &mut self.channels_mut()[2]
    }
}

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<FrontL> for Frame<Chan, 4> {
    type Output = Chan;

    fn index(&self, _: FrontL) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<FrontL> for Frame<Chan, 4> {
    fn index_mut(&mut self, _: FrontL) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

impl<Chan: Channel> Index<FrontR> for Frame<Chan, 4> {
    type Output = Chan;

    fn index(&self, _: FrontR) -> &Self::Output {
        &self.channels()[1]
    }
}

impl<Chan: Channel> IndexMut<FrontR> for Frame<Chan, 4> {
    fn index_mut(&mut self, _: FrontR) -> &mut Self::Output {
        &mut self.channels_mut()[1]
    }
}

impl<Chan: Channel> Index<SurroundL> for Frame<Chan, 4> {
    type Output = Chan;

    fn index(&self, _: SurroundL) -> &Self::Output {
        &self.channels()[2]
    }
}

impl<Chan: Channel> IndexMut<SurroundL> for Frame<Chan, 4> {
    fn index_mut(&mut self, _: SurroundL) -> &mut Self::Output {
        &mut self.channels_mut()[2]
    }
}

impl<Chan: Channel> Index<SurroundR> for Frame<Chan, 4> {
    type Output = Chan;

    fn index(&self, _: SurroundR) -> &Self::Output {
        &self.channels()[3]
    }
}

impl<Chan: Channel> IndexMut<SurroundR> for Frame<Chan, 4> {
    fn index_mut(&mut self, _: SurroundR) -> &mut Self::Output {
        &mut self.channels_mut()[3]
    }
}

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<FrontL> for Frame<Chan, 5> {
    type Output = Chan;

    fn index(&self, _: FrontL) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<FrontL> for Frame<Chan, 5> {
    fn index_mut(&mut self, _: FrontL) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

impl<Chan: Channel> Index<FrontR> for Frame<Chan, 5> {
    type Output = Chan;

    fn index(&self, _: FrontR) -> &Self::Output {
        &self.channels()[1]
    }
}

impl<Chan: Channel> IndexMut<FrontR> for Frame<Chan, 5> {
    fn index_mut(&mut self, _: FrontR) -> &mut Self::Output {
        &mut self.channels_mut()[1]
    }
}

impl<Chan: Channel> Index<Front> for Frame<Chan, 5> {
    type Output = Chan;

    fn index(&self, _: Front) -> &Self::Output {
        &self.channels()[2]
    }
}

impl<Chan: Channel> IndexMut<Front> for Frame<Chan, 5> {
    fn index_mut(&mut self, _: Front) -> &mut Self::Output {
        &mut self.channels_mut()[2]
    }
}

impl<Chan: Channel> Index<SurroundL> for Frame<Chan, 5> {
    type Output = Chan;

    fn index(&self, _: SurroundL) -> &Self::Output {
        &self.channels()[3]
    }
}

impl<Chan: Channel> IndexMut<SurroundL> for Frame<Chan, 5> {
    fn index_mut(&mut self, _: SurroundL) -> &mut Self::Output {
        &mut self.channels_mut()[3]
    }
}

impl<Chan: Channel> Index<SurroundR> for Frame<Chan, 5> {
    type Output = Chan;

    fn index(&self, _: SurroundR) -> &Self::Output {
        &self.channels()[4]
    }
}

impl<Chan: Channel> IndexMut<SurroundR> for Frame<Chan, 5> {
    fn index_mut(&mut self, _: SurroundR) -> &mut Self::Output {
        &mut self.channels_mut()[4]
    }
}

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<FrontL> for Frame<Chan, 6> {
    type Output = Chan;

    fn index(&self, _: FrontL) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<FrontL> for Frame<Chan, 6> {
    fn index_mut(&mut self, _: FrontL) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

impl<Chan: Channel> Index<FrontR> for Frame<Chan, 6> {
    type Output = Chan;

    fn index(&self, _: FrontR) -> &Self::Output {
        &self.channels()[1]
    }
}

impl<Chan: Channel> IndexMut<FrontR> for Frame<Chan, 6> {
    fn index_mut(&mut self, _: FrontR) -> &mut Self::Output {
        &mut self.channels_mut()[1]
    }
}

impl<Chan: Channel> Index<Front> for Frame<Chan, 6> {
    type Output = Chan;

    fn index(&self, _: Front) -> &Self::Output {
        &self.channels()[2]
    }
}

impl<Chan: Channel> IndexMut<Front> for Frame<Chan, 6> {
    fn index_mut(&mut self, _: Front) -> &mut Self::Output {
        &mut self.channels_mut()[2]
    }
}

impl<Chan: Channel> Index<Lfe> for Frame<Chan, 6> {
    type Output = Chan;

    fn index(&self, _: Lfe) -> &Self::Output {
        &self.channels()[3]
    }
}

impl<Chan: Channel> IndexMut<Lfe> for Frame<Chan, 6> {
    fn index_mut(&mut self, _: Lfe) -> &mut Self::Output {
        &mut self.channels_mut()[3]
    }
}

impl<Chan: Channel> Index<SurroundL> for Frame<Chan, 6> {
    type Output = Chan;

    fn index(&self, _: SurroundL) -> &Self::Output {
        &self.channels()[4]
    }
}

impl<Chan: Channel> IndexMut<SurroundL> for Frame<Chan, 6> {
    fn index_mut(&mut self, _: SurroundL) -> &mut Self::Output {
        &mut self.channels_mut()[4]
    }
}

impl<Chan: Channel> Index<SurroundR> for Frame<Chan, 6> {
    type Output = Chan;

    fn index(&self, _: SurroundR) -> &Self::Output {
        &self.channels()[5]
    }
}

impl<Chan: Channel> IndexMut<SurroundR> for Frame<Chan, 6> {
    fn index_mut(&mut self, _: SurroundR) -> &mut Self::Output {
        &mut self.channels_mut()[5]
    }
}

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<FrontL> for Frame<Chan, 7> {
    type Output = Chan;

    fn index(&self, _: FrontL) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<FrontL> for Frame<Chan, 7> {
    fn index_mut(&mut self, _: FrontL) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

impl<Chan: Channel> Index<FrontR> for Frame<Chan, 7> {
    type Output = Chan;

    fn index(&self, _: FrontR) -> &Self::Output {
        &self.channels()[1]
    }
}

impl<Chan: Channel> IndexMut<FrontR> for Frame<Chan, 7> {
    fn index_mut(&mut self, _: FrontR) -> &mut Self::Output {
        &mut self.channels_mut()[1]
    }
}

impl<Chan: Channel> Index<Front> for Frame<Chan, 7> {
    type Output = Chan;

    fn index(&self, _: Front) -> &Self::Output {
        &self.channels()[2]
    }
}

impl<Chan: Channel> IndexMut<Front> for Frame<Chan, 7> {
    fn index_mut(&mut self, _: Front) -> &mut Self::Output {
        &mut self.channels_mut()[2]
    }
}

impl<Chan: Channel> Index<Lfe> for Frame<Chan, 7> {
    type Output = Chan;

    fn index(&self, _: Lfe) -> &Self::Output {
        &self.channels()[3]
    }
}

impl<Chan: Channel> IndexMut<Lfe> for Frame<Chan, 7> {
    fn index_mut(&mut self, _: Lfe) -> &mut Self::Output {
        &mut self.channels_mut()[3]
    }
}

impl<Chan: Channel> Index<Back> for Frame<Chan, 7> {
    type Output = Chan;

    fn index(&self, _: Back) -> &Self::Output {
        &self.channels()[4]
    }
}

impl<Chan: Channel> IndexMut<Back> for Frame<Chan, 7> {
    fn index_mut(&mut self, _: Back) -> &mut Self::Output {
        &mut self.channels_mut()[4]
    }
}

impl<Chan: Channel> Index<Left> for Frame<Chan, 7> {
    type Output = Chan;

    fn index(&self, _: Left) -> &Self::Output {
        &self.channels()[5]
    }
}

impl<Chan: Channel> IndexMut<Left> for Frame<Chan, 7> {
    fn index_mut(&mut self, _: Left) -> &mut Self::Output {
        &mut self.channels_mut()[5]
    }
}

impl<Chan: Channel> Index<Right> for Frame<Chan, 7> {
    type Output = Chan;

    fn index(&self, _: Right) -> &Self::Output {
        &self.channels()[6]
    }
}

impl<Chan: Channel> IndexMut<Right> for Frame<Chan, 7> {
    fn index_mut(&mut self, _: Right) -> &mut Self::Output {
        &mut self.channels_mut()[6]
    }
}

////////////////////////////////////////////////////////////

impl<Chan: Channel> Index<FrontL> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: FrontL) -> &Self::Output {
        &self.channels()[0]
    }
}

impl<Chan: Channel> IndexMut<FrontL> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: FrontL) -> &mut Self::Output {
        &mut self.channels_mut()[0]
    }
}

impl<Chan: Channel> Index<FrontR> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: FrontR) -> &Self::Output {
        &self.channels()[1]
    }
}

impl<Chan: Channel> IndexMut<FrontR> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: FrontR) -> &mut Self::Output {
        &mut self.channels_mut()[1]
    }
}

impl<Chan: Channel> Index<Front> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: Front) -> &Self::Output {
        &self.channels()[2]
    }
}

impl<Chan: Channel> IndexMut<Front> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: Front) -> &mut Self::Output {
        &mut self.channels_mut()[2]
    }
}

impl<Chan: Channel> Index<Lfe> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: Lfe) -> &Self::Output {
        &self.channels()[3]
    }
}

impl<Chan: Channel> IndexMut<Lfe> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: Lfe) -> &mut Self::Output {
        &mut self.channels_mut()[3]
    }
}

impl<Chan: Channel> Index<BackL> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: BackL) -> &Self::Output {
        &self.channels()[4]
    }
}

impl<Chan: Channel> IndexMut<BackL> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: BackL) -> &mut Self::Output {
        &mut self.channels_mut()[4]
    }
}

impl<Chan: Channel> Index<BackR> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: BackR) -> &Self::Output {
        &self.channels()[5]
    }
}

impl<Chan: Channel> IndexMut<BackR> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: BackR) -> &mut Self::Output {
        &mut self.channels_mut()[5]
    }
}

impl<Chan: Channel> Index<Left> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: Left) -> &Self::Output {
        &self.channels()[6]
    }
}

impl<Chan: Channel> IndexMut<Left> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: Left) -> &mut Self::Output {
        &mut self.channels_mut()[6]
    }
}

impl<Chan: Channel> Index<Right> for Frame<Chan, 8> {
    type Output = Chan;

    fn index(&self, _: Right) -> &Self::Output {
        &self.channels()[7]
    }
}

impl<Chan: Channel> IndexMut<Right> for Frame<Chan, 8> {
    fn index_mut(&mut self, _: Right) -> &mut Self::Output {
        &mut self.channels_mut()[7]
    }
}
