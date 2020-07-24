use super::embedded_hal::blocking::spi::Write;

impl embedded_hal::blocking::spi::Write<u8> for crate::spi::Spi {
    type Error = i32;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.tx(words)
    }
}
