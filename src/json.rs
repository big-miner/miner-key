use serde_json::ser::Formatter;
use std::io;
use anyhow::Result;

macro_rules! tri {
    ($e:expr) => {
        match $e {
            Result::Ok(val) => val,
            Result::Err(_err) => (),
        }
    };
    ($e:expr,) => {
        tri!($e)
    };
}


#[derive(Clone, Debug)]
pub struct JsonPrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}

impl<'a> JsonPrettyFormatter<'a> {
    /// Construct a pretty printer formatter that defaults to using four spaces for indentation.
    pub fn new() -> Self {
        JsonPrettyFormatter::with_indent(b"    ")
    }

    /// Construct a pretty printer formatter that uses the `indent` string for indentation.
    pub fn with_indent(indent: &'a [u8]) -> Self {
        JsonPrettyFormatter {
            current_indent: 0,
            has_value: false,
            indent,
        }
    }
}


impl<'a> Formatter for JsonPrettyFormatter<'a> {
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        writer.write_all(b"\x1B[38;2;238;44;44mnull\x1B[0m")
    }

    /// Writes a `true` or `false` value to the specified writer.
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        let s = if value {
            // b"true" as &[u8]
            b"\x1B[38;2;138;43;226mtrue\x1B[0m" as &[u8]
        } else {
            // b"false" as &[u8]
            b"\x1B[38;2;138;43;226mfalse\x1B[0m" as &[u8]
        };
        writer.write_all(s)
    }


    /// Writes a number that has already been rendered to a string.
    #[inline]
    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        let p = format!("\x1B[38;2;238;44;44m{:?}\x1B[0m", value);
        writer.write_all(p.as_bytes())
    }


    /// Called before each series of `write_string_fragment` and
    /// `write_char_escape`.  Writes a `"` to the specified writer.
    #[inline]
    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        writer.write_all(b"\x1B[37m\"\x1B[0m")
    }

    /// Called after each series of `write_string_fragment` and
    /// `write_char_escape`.  Writes a `"` to the specified writer.
    #[inline]
    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        writer.write_all(b"\x1B[37m\"\x1B[0m")
    }

    /// Writes a string fragment that doesn't need any escaping to the
    /// specified writer.
    #[inline]
    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        let p = format!("\x1B[38;2;50;205;50m{}\x1B[0m", fragment);
        writer.write_all(p.as_bytes())
    }
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"\x1B[37m[\x1B[0m")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            tri!(writer.write_all(b"\n"));
            tri!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"\x1B[37m]\x1B[0m")
    }


    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        if first {
            tri!(writer.write_all(b"\n"));
        } else {
            // tri!(writer.write_all(b",\n"));
            tri!(writer.write_all(b"\x1B[37m,\x1B[0m\n"));
        }
        tri!(indent(writer, self.current_indent, self.indent));
        Ok(())
    }

    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"\x1B[37m{\x1B[0m")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            tri!(writer.write_all(b"\n"));
            tri!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"\x1B[37m}\x1B[0m")
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        if first {
            tri!(writer.write_all(b"\n"));
        } else {
            tri!(writer.write_all(b"\x1B[37m,\x1B[0m\n"));
        }
        indent(writer, self.current_indent, self.indent)
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        writer.write_all(b"\x1B[37m: \x1B[0m")
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }
}


fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
    where
        W: ?Sized + io::Write,
{
    for _ in 0..n {
        tri!(wr.write_all(s));
    }

    Ok(())
}