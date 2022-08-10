use std::{io::Error, path::Path};
use color_eyre::Result;
use naga::{back::spv::{self, BindingMap}, front::wgsl, valid::Validator, WithSpan};
use naga::valid::{Capabilities, ValidationError, ValidationFlags};


pub struct ShaderCompiler {
    parser: wgsl::Parser,
    validator: Validator,
    writer: spv::Writer,
}

impl ShaderCompiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_shader_module(&mut self, path: &Path) -> Result<Vec<u32>, CompilerError> {
        let source = std::fs::read_to_string(path)?;
        let module = self
            .parser
            .parse(&source)
            .map_err(|error| CompilerError::Compile { error, source })?;
        let module_info = self.validator.validate(&module)?;
        let mut words = vec![];
        self.writer.write(&module, &module_info, None, &mut words)?;
        Ok(words)
    }
}

impl Default for ShaderCompiler {
    fn default() -> Self {
        let parser = wgsl::Parser::new();
        let validator = Validator::new(ValidationFlags::all(), Capabilities::all());
        let options = get_options();
        let writer = spv::Writer::new(&options).unwrap();
        Self {
            parser,
            validator,
            writer,
        }
    }
}

fn get_options() -> spv::Options {
    let capabilities = vec![
        spv::Capability::Shader,
        spv::Capability::Matrix,
        spv::Capability::Sampled1D,
        spv::Capability::Image1D,
        spv::Capability::ImageQuery,
        spv::Capability::ImageQuery,
        spv::Capability::DerivativeControl,
        spv::Capability::SampledCubeArray,
        spv::Capability::SampleRateShading,
        //不用在意真正的适配器支持什么存储格式，这和spv-out翻译程序无关
        spv::Capability::StorageImageExtendedFormats,
        spv::Capability::MultiView,
    ];

    let mut flags = spv::WriterFlags::empty();
    flags.set(
        spv::WriterFlags::DEBUG,
        true,
    );
    flags.set(
        spv::WriterFlags::LABEL_VARYINGS,
        true,
    );
    flags.set(
        spv::WriterFlags::FORCE_POINT_SIZE,
        true,
    );
    spv::Options {
        binding_map: BindingMap::new(),
        lang_version: (1, 0),
        flags,
        capabilities: Some(capabilities.into_iter().collect()),
        bounds_check_policies: naga::proc::BoundsCheckPolicies {
            index: naga::proc::BoundsCheckPolicy::Unchecked,
            buffer: naga::proc::BoundsCheckPolicy::Unchecked,
            image: naga::proc::BoundsCheckPolicy::Unchecked,
            binding_array: naga::proc::BoundsCheckPolicy::Unchecked,
        },
    }
}


pub enum CompilerError {
    Read(std::io::Error),
    Compile {
        error: wgsl::ParseError,
        source: String,
    },
    Validate(naga::WithSpan<ValidationError>),
    WriteSpirv(spv::Error),
}

impl From<std::io::Error> for CompilerError {
    fn from(e: std::io::Error) -> Self {
        Self::Read(e)
    }
}

impl From<naga::WithSpan<ValidationError>> for CompilerError {
    fn from(e: WithSpan<ValidationError>) -> Self {
        Self::Validate(e)
    }
}

impl From<spv::Error> for CompilerError {
    fn from(e: spv::Error) -> Self {
        Self::WriteSpirv(e)
    }
}


impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(err) => write!(f, "{}", err),
            Self::WriteSpirv(err) => write!(f, "{:?}", err),
            Self::Validate(err) => write!(f, "{:?}", err),
            Self::Compile { error, source } => {
                error.emit_to_stderr(source);
                Ok(())
            }
        }
    }
}

impl std::fmt::Debug for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(err) => write!(f, "{}", err),
            Self::WriteSpirv(err) => write!(f, "{:?}", err),
            Self::Validate(err) => write!(f, "{:?}", err),
            Self::Compile { error, source } => write!(f, "{}", error.emit_to_string(source)),
        }
    }
}

impl std::error::Error for CompilerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Read(ref e) => Some(e),
            Self::Compile { error: ref e, .. } => Some(e),
            Self::Validate(ref e) => Some(e),
            Self::WriteSpirv(ref e) => Some(e),
        }
    }
}
