use crate::{
    map::position::Position,
    network::packet::PacketOut,
    player::{Gender, OutfitColors},
};
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};


/********************************************************************************
 * 
 * Read helpers
 *
 * Utilities for decoding Tibia 1.03 network packets.
 * 
 ********************************************************************************/
impl<R: AsyncRead + Unpin> ReadExt for R {}

pub trait ReadExt: AsyncRead + Unpin + Sized {
    /********************************************************************************
     * 
     * Packed 4-bit values
     * 
     ********************************************************************************/
    async fn read_u4(&mut self) -> Result<(u8, u8)> {
        let byte = self.read_u8().await?;
        Ok((byte / 16, byte % 16))
    }


    /********************************************************************************
     * 
     * Fixed-size null terminated string
     * 
     * Reads characters until a null terminator is encountered,
     * then skips the remaining bytes in the field.
     * 
     ********************************************************************************/
    async fn read_string(&mut self, buf: &mut String, max_size: u16) -> Result<usize> {
        for n in 1..=max_size {
            match self.read_u8().await? {
                b'\0' => {
                    self.skip(max_size - n).await?;
                    break;
                }
                c => buf.push(c as char),
            }
        }
        Ok(buf.len())
    }


    /********************************************************************************
     * 
     * Skip bytes
     * 
     ********************************************************************************/
    async fn skip(&mut self, bytes: u16) -> Result<()> {
        let mut buf = vec![0_u8; bytes as usize];
        self.read_exact(&mut buf).await?;
        Ok(())
    }


    /********************************************************************************
     * 
     * Null terminated string
     * 
     * Reads until EOF or null terminator.
     * 
     ********************************************************************************/
    async fn read_string_to_end(&mut self, buf: &mut String) -> Result<()> {
        let mut byte = [0u8; 1];
        loop {
            match self.read_exact(&mut byte).await {
                Ok(_) if byte[0] == 0 => break,
                Ok(_) => buf.push(byte[0] as char),
                Err(_) => break,
            }
        }
        Ok(())
    }


    /********************************************************************************
     * 
     * Map position
     * 
     * Tibia 1.03 positions contain only X and Y coordinates.
     * Floor level (Z) is always 7.
     * 
     ********************************************************************************/
    async fn read_position(&mut self) -> Result<Position> {
        let x = self.read_u8().await? as u16;
        let y = self.read_u8().await? as u16;
        Ok(Position::new(x, y, 7))
    }


    /********************************************************************************
     * 
     * Outfit colors
     *
     * Tibia 1.03 stores four outfit colors packed into two bytes:
     * - Byte 1: legs (high nibble), shoes (low nibble)
     * - Byte 2: head (high nibble), body (low nibble)
     * 
     ********************************************************************************/
    async fn read_outfit_colors(&mut self) -> Result<OutfitColors> {
        let (legs, shoes) = self.read_u4().await?;
        let (head, body)  = self.read_u4().await?;
        let _ = self.read_u8().await?;
        Ok(OutfitColors::new(head, body, legs, shoes))
    }


    /********************************************************************************
     * 
     * Gender
     * 
     ********************************************************************************/
    async fn read_gender(&mut self) -> Result<Gender> {
        Ok(match self.read_u8().await? {
            1 => Gender::Male,
            _ => Gender::Female,
        })
    }
}


/********************************************************************************
 * 
 * Write helpers
 *
 * Utilities for encoding Tibia 1.03 network packets
 * 
 ********************************************************************************/
impl<W: AsyncWrite + Unpin> WriteExt for W {}

pub trait WriteExt: AsyncWrite + Unpin + Sized {
    /********************************************************************************
     * 
     * Packet header
     * 
     * Tibia 1.03 packets begin with four zero bytes followed by a one-bye packet identifier.
     * 
     ********************************************************************************/
    async fn write_packet(&mut self, packet: PacketOut) -> Result<()> {
        self.write_u32_le(0).await?;
        self.write_u8(packet as u8).await?;
        Ok(())
    }


    /********************************************************************************
     * 
     * Packed 4-bit values
     * 
     ********************************************************************************/
    async fn write_u4(&mut self, high: u8, low: u8) -> Result<()> {
        self.write_u8((high << 4) + low).await?;
        Ok(())
    }


    /********************************************************************************
     * 
     * Null terminated string
     * 
     ********************************************************************************/
    async fn write_null_terminated_string(&mut self, s: &str) -> Result<()> {
        self.write_all(s.as_bytes()).await?;
        self.write_u8(0).await?;
        Ok(())
    }


    /********************************************************************************
     * 
     * Fixed-size string
     * 
     * Writes a string padded with trailing zeroes to a fixed length.
     * 
     ********************************************************************************/
    async fn write_fixed_string(&mut self, s: &str, length: u16) -> Result<()> {
        let len = length as usize;
        if s.len() >= len {
            self.write_all(&s.as_bytes()[..len]).await?;
        } else {
            self.write_all(s.as_bytes()).await?;
            self.write_zeroes(len - s.len()).await?;
        }
        Ok(())
    }


    /********************************************************************************
     * 
     * Zero padding
     * 
     ********************************************************************************/
    async fn write_zeroes(&mut self, count: usize) -> Result<()> {
        self.write_all(&vec![0u8; count]).await?;
        Ok(())
    }


    /********************************************************************************
     * 
     * Map position
     * 
     * Tibia 1.03 positions only contain X and Y coordinates.
     * 
     ********************************************************************************/
    async fn write_position(&mut self, pos: Position) -> Result<()> {
        self.write_u8(pos.x as u8).await?;
        self.write_u8(pos.y as u8).await?;
        Ok(())
    }


    /********************************************************************************
     * 
     * Outfit colors
     * 
     ********************************************************************************/
    async fn write_outfit_colors(&mut self, outfit: OutfitColors) -> Result<()> {
        self.write_u4(outfit.legs, outfit.shoes).await?;
        self.write_u4(outfit.head, outfit.body).await?;
        self.write_u8(0).await?;
        Ok(())
    }


    /********************************************************************************
     * 
     * Gender
     * 
     ********************************************************************************/
    async fn write_gender(&mut self, gender: Gender) -> Result<()> {
        self.write_u8(match gender {
            Gender::Male   => 1,
            Gender::Female => 0,
        }).await?;
        Ok(())
    }
}