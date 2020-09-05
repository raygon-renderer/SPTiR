var searchIndex = JSON.parse('{\
"chapter_0_prelude":{"doc":"Spectral Path Tracing in Rust: PreludeRay tracing is all…","i":[],"p":[]},\
"main":{"doc":"","i":[[5,"main","main","",null,[[]]]],"p":[]},\
"sptir":{"doc":"Spectral Path Tracing in Rust","i":[[0,"color","sptir","",null,null],[3,"ColorMatrix3","sptir::color","",null,null],[12,"0","","",0,null],[3,"xyY","","A two-dimensional coordinate on the xy-chromaticity…",null,null],[12,"x","","",1,null],[12,"y","","",1,null],[12,"luminance","","",1,null],[3,"Adaptation","","Helper struct for Chromatic Adaptation, converting from…",null,null],[3,"RGBColor","","Color consisting of Red, Green and Blue components,…",null,null],[12,"r","","",2,null],[12,"g","","",2,null],[12,"b","","",2,null],[3,"RGBPrimaries","","Defines the coordinates in the xy-chromaticity diagram for…",null,null],[12,"r","","",3,null],[12,"g","","",3,null],[12,"b","","",3,null],[5,"compute_adaptation_matrix","","Computes a linear transform to adapt one whitepoint to…",null,[[["xyy",3]],["colormatrix3",3]]],[18,"IDENTITY","","",0,null],[11,"inverse","","",0,[[],["colormatrix3",3]]],[11,"multiply","","",0,[[["colormatrix3",3]],["colormatrix3",3]]],[11,"transform_xyz","","",0,[[["xyzspectrum",3]],["xyzspectrum",3]]],[11,"transform_rgb","","WARNING: This method does not actually calculate the color…",0,[[["colorspace",8],["rgbcolor",3]],[["colorspace",8],["rgbcolor",3]]]],[11,"transpose","","",0,[[],["colormatrix3",3]]],[0,"spaces","","",null,null],[3,"ACES_AP0","sptir::color::spaces","ACES 2065-1 Extreme Wide-Gamut Color Space",null,null],[3,"ACES_AP1","","ACEScg Wide-Gamut Color Space",null,null],[3,"SRGB","","Rec701 sRGB Color Space",null,null],[6,"SRGBColor","","",null,null],[6,"ACES2065_1Color","","",null,null],[6,"ACEScgColor","","",null,null],[11,"decode_transfer_function_from","sptir::color","Decodes an 8-bit encoded sRGB color.",2,[[],["srgbcolor",6]]],[11,"encode_transfer_function_to","","Encodes a linear sRGB color to an output slice of 8-bit…",2,[[]]],[11,"decode_transfer_function_from","sptir::color::spaces","Decodes an 8-bit encoded sRGB color.",4,[[],["srgbcolor",6]]],[11,"encode_transfer_function_to","","Encodes a linear sRGB color to an output slice of 8-bit…",4,[[]]],[0,"whitepoints","sptir::color","",null,null],[17,"D65","sptir::color::whitepoints","Noon Daylight: Television, sRGB color space",null,null],[17,"D60","","D60 has a CCT of approximately 6000K",null,null],[17,"ACES","","D60-like whitepoint designed for future-compatibility of…",null,null],[17,"FORWARD_BRADFORD_MATRIX","sptir::color","Bradford cone response domain matrix",null,null],[17,"INVERSE_BRADFORD_MATRIX","","Inverse of `FORWARD_BRADFORD_MATRIX`, computed…",null,null],[8,"Colorspace","","Defines a working space within the human chromaticity…",null,null],[18,"PRIMARIES","","Defines the coordinates in the xy-chromaticity diagram for…",5,null],[18,"WHITEPOINT","","Defines the standard illuminant for the color space",5,null],[18,"RGB_TO_XYZ","","Conversion matrix from Linear RGB to XYZ tristimulus values",5,null],[18,"XYZ_TO_RGB","","Conversion matrix from XYZ tristimulus values to Linear RGB",5,null],[11,"new","","",1,[[],["xyy",3]]],[11,"to_xyz","","Converts xyY to XYZ tristimulus values",1,[[],["xyzspectrum",3]]],[18,"XYZ_TO_XYZ","","",6,null],[18,"RGB_TO_RGB","","",6,null],[11,"new","","",2,[[],["rgbcolor",3]]],[11,"convert","","Convert the RGB value of one colorspace to an RGB value in…",2,[[],[["colorspace",8],["rgbcolor",3]]]],[11,"to_xyz","","",2,[[],["xyzspectrum",3]]],[11,"rgb_to_xyz","","Derives the linear transformation matrix…",3,[[["xyy",3]],["colormatrix3",3]]],[0,"film","sptir","",null,null],[3,"Film","sptir::film","",null,null],[12,"pixels","","",7,null],[12,"width","","",7,null],[12,"height","","",7,null],[11,"new","","",7,[[],["film",3]]],[11,"put","","",7,[[["xyzspectrum",3]]]],[11,"save_as","","",7,[[["path",3],["asref",8]],[["result",4],["imageerror",4]]]],[0,"geometry","sptir","",null,null],[3,"Matrix4","sptir::geometry","A 4x4 matrix",null,null],[12,"0","","",8,null],[3,"Point3","","",null,null],[12,"x","","",9,null],[12,"y","","",9,null],[12,"z","","",9,null],[3,"Ray","","",null,null],[12,"origin","","",10,null],[12,"direction","","",10,null],[12,"tmax","","",10,null],[3,"Transform3","","",null,null],[12,"forward","","",11,null],[12,"inverse","","",11,null],[3,"Vector3","","",null,null],[12,"x","","",12,null],[12,"y","","",12,null],[12,"z","","",12,null],[18,"IDENTITY","","The Identity matrix",8,null],[18,"ZERO","","Matrix initialized to all zeroes.",8,null],[11,"new","","",8,[[],["matrix4",3]]],[11,"transpose","","Compute `$\\\\begin{bmatrix}M\\\\end{bmatrix}^T$`:",8,[[],["matrix4",3]]],[18,"ORIGIN","","",9,null],[11,"new","","",9,[[],["point3",3]]],[11,"as_vector","","",9,[[],["vector3",3]]],[11,"at","","",10,[[],["point3",3]]],[11,"invert","","",11,[[],["transform3",3]]],[18,"ZERO","","",12,null],[18,"UP","","",12,null],[11,"new","","",12,[[],["vector3",3]]],[11,"dot","","The dot product (or scalar product) of two N-dimensional…",12,[[]]],[11,"cross","","The cross product (or vector product) is a binary…",12,[[],["vector3",3]]],[11,"norm_squared","","",12,[[]]],[11,"norm","","Compute the Euclidean norm (`$\\\\|\\\\mathbf{v}\\\\|$`) of the…",12,[[]]],[11,"normalize","","Normalizes the vector such that the 3D Euclidean norm is 1.0",12,[[],["vector3",3]]],[0,"math","sptir","",null,null],[5,"integrate","sptir::math","Computes `$\\\\int_a^b f(x) dx$` with an adaptive Simpson\'s…",null,[[]]],[0,"sampling","sptir","",null,null],[3,"Sample1D","sptir::sampling","",null,null],[12,"0","","",13,null],[3,"Sample2D","","",null,null],[12,"u","","",14,null],[12,"v","","",14,null],[3,"IndependentSampler","","",null,null],[4,"SampleDimension1D","","",null,null],[13,"Wavelength","","",15,null],[13,"Time","","",15,null],[13,"LightPick","","",15,null],[13,"Terminate","","",15,null],[4,"SampleDimension2D","","",null,null],[13,"Film","","",16,null],[13,"Lens","","",16,null],[13,"BSDF","","",16,null],[13,"Light","","",16,null],[8,"Sampler","","",null,null],[10,"set_sample","","",17,[[]]],[10,"set_pixel","","",17,[[]]],[10,"next1d","","",17,[[["sampledimension1d",4]],["sample1d",3]]],[10,"next2d","","",17,[[["sampledimension2d",4]],["sample2d",3]]],[11,"new","","",18,[[],["independentsampler",3]]],[0,"spectrum","sptir","",null,null],[0,"distributions","sptir::spectrum","Spectral Power and Reflectance Distributions",null,null],[0,"illuminant_d","sptir::spectrum::distributions","Standard Illuminant D-series",null,null],[3,"DSeries","sptir::spectrum::distributions::illuminant_d","",null,null],[11,"new","","",19,[[],[["option",4],["dseries",3]]]],[11,"sample","","",19,[[]]],[0,"sampled","sptir::spectrum::distributions","Arbitrary sampled spectral distributions",null,null],[3,"PiecewiseLinearSpectrum","sptir::spectrum::distributions::sampled","",null,null],[12,"lambda_y","","",20,null],[11,"new","","",20,[[["vec",3]],["piecewiselinearspectrum",3]]],[11,"from_fn","","",20,[[],["piecewiselinearspectrum",3]]],[11,"sample","","",20,[[]]],[8,"SpectralPowerDistribution","sptir::spectrum::distributions","`$L_{\\\\mathbf{a},\\\\Omega,\\\\lambda}$`, spectral radiance",null,null],[10,"radiance","","",21,[[["spectralwavelengths",3]],["spectralradiance",3]]],[8,"SpectralReflectanceDistribution","","`$R_{\\\\lambda}$`, unitless reflectance factor",null,null],[10,"reflectance","","Computes `$R_{\\\\lambda}$`, which should always be between 0…",22,[[["spectralwavelengths",3]],["spectralreflectance",3]]],[0,"range","sptir::spectrum","Range `$\\\\int_{\\\\lambda_{min}}^{\\\\lambda_{max}}…",null,null],[3,"SpectralRange","sptir::spectrum::range","Range `$\\\\int_{\\\\lambda_{min}}^{\\\\lambda_{max}}…",null,null],[12,"min","","`$\\\\lambda_{min}$`, the minimum wavelength which can be…",23,null],[12,"max","","`$\\\\lambda_{max}$`, the maximum wavelength which can be…",23,null],[12,"y_integral","","Normalizing factor",23,null],[18,"VISIBLE","","Precomputed visible wavelength range between 360 and 840…",23,null],[11,"new","","Creates a new spectral range between `$\\\\lambda_{min}$` and…",23,[[],["spectralrange",3]]],[11,"hero_to_xyz","","",23,[[["spectralflux",3],["spectralwavelengths",3]],["xyzspectrum",3]]],[11,"sample","","Samples a hero wavelength and `NumLanes - 1` number of…",23,[[],["spectralwavelengths",3]]],[0,"units","sptir::spectrum","Spectral Units (Wavelength `$\\\\lambda_h$`, Flux…",null,null],[3,"SpectralWavelengths","sptir::spectrum::units","`$\\\\lambda_h$` Hero wavelength spectrum samples with PDF",null,null],[12,"lambda","","The sampled wavelengths (`$\\\\lambda$`), where `$\\\\lambda_h =…",24,null],[12,"pdf","","The sampling probability for this wavelength",24,null],[3,"SpectralRadiance","","`$L_{\\\\mathbf{a},\\\\Omega,\\\\lambda}$`, surface radiance in…",null,null],[12,"energy","","",25,null],[3,"SpectralReflectance","","`$R_{\\\\lambda}$`, unitless reflectance factor between 0 and 1",null,null],[12,"refl","","",26,null],[3,"SpectralFlux","","`$\\\\Phi_{\\\\mathbf{e},\\\\lambda}$`, radiant energy in watts per…",null,null],[12,"energy","","",27,null],[6,"NumLanes","","Defines how many wavelengths should be used for HWSS",null,null],[6,"Lanes","","Helper type to define an array of values correspoding to…",null,null],[11,"hero","","Hero wavelength `$\\\\lambda_h$`",24,[[]]],[0,"xyz","sptir::spectrum","XYZ Tristumulus values and basis functions",null,null],[3,"XYZSpectrum","sptir::spectrum::xyz","XYZ Tristimulus color values",null,null],[12,"x","","",28,null],[12,"y","","",28,null],[12,"z","","",28,null],[5,"x_bar","","`$\\\\overline{x}(\\\\lambda)$` basis function approximation",null,[[]]],[5,"y_bar","","`$\\\\overline{y}(\\\\lambda)$` basis function approximation",null,[[]]],[5,"z_bar","","`$\\\\overline{z}(\\\\lambda)$` basis function approximation",null,[[]]],[18,"ZERO","","",28,null],[11,"new","","",28,[[],["xyzspectrum",3]]],[11,"to_rgb","","Converts XYZ tristimulus values to RGB within a certain…",28,[[],[["rgbcolor",3],["colorspace",8]]]],[11,"from_wavelength","","Calculates the tristimulus response values for the given…",28,[[],["xyzspectrum",3]]],[0,"world","sptir","",null,null],[0,"aggregate","sptir::world","",null,null],[4,"Aggregate","sptir::world::aggregate","",null,null],[0,"primitive","sptir::world","",null,null],[0,"sphere","sptir::world::primitive","",null,null],[11,"from","sptir::color","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"from","sptir::color::spaces","",29,[[]]],[11,"into","","",29,[[]]],[11,"try_from","","",29,[[],["result",4]]],[11,"try_into","","",29,[[],["result",4]]],[11,"borrow","","",29,[[]]],[11,"borrow_mut","","",29,[[]]],[11,"type_id","","",29,[[],["typeid",3]]],[11,"vzip","","",29,[[]]],[11,"from","","",30,[[]]],[11,"into","","",30,[[]]],[11,"try_from","","",30,[[],["result",4]]],[11,"try_into","","",30,[[],["result",4]]],[11,"borrow","","",30,[[]]],[11,"borrow_mut","","",30,[[]]],[11,"type_id","","",30,[[],["typeid",3]]],[11,"vzip","","",30,[[]]],[11,"from","","",31,[[]]],[11,"into","","",31,[[]]],[11,"try_from","","",31,[[],["result",4]]],[11,"try_into","","",31,[[],["result",4]]],[11,"borrow","","",31,[[]]],[11,"borrow_mut","","",31,[[]]],[11,"type_id","","",31,[[],["typeid",3]]],[11,"vzip","","",31,[[]]],[11,"from","sptir::film","",7,[[]]],[11,"into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"from","sptir::geometry","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"vzip","","",11,[[]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"vzip","","",12,[[]]],[11,"from","sptir::sampling","",13,[[]]],[11,"into","","",13,[[]]],[11,"to_owned","","",13,[[]]],[11,"clone_into","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"vzip","","",13,[[]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"vzip","","",14,[[]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"vzip","","",18,[[]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"vzip","","",15,[[]]],[11,"from","","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"vzip","","",16,[[]]],[11,"from","sptir::spectrum::distributions::illuminant_d","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"vzip","","",19,[[]]],[11,"from","sptir::spectrum::distributions::sampled","",20,[[]]],[11,"into","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"vzip","","",20,[[]]],[11,"from","sptir::spectrum::range","",23,[[]]],[11,"into","","",23,[[]]],[11,"to_owned","","",23,[[]]],[11,"clone_into","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"vzip","","",23,[[]]],[11,"from","sptir::spectrum::units","",24,[[]]],[11,"into","","",24,[[]]],[11,"to_owned","","",24,[[]]],[11,"clone_into","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"vzip","","",24,[[]]],[11,"from","","",25,[[]]],[11,"into","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"vzip","","",25,[[]]],[11,"from","","",26,[[]]],[11,"into","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"try_into","","",26,[[],["result",4]]],[11,"borrow","","",26,[[]]],[11,"borrow_mut","","",26,[[]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"vzip","","",26,[[]]],[11,"from","","",27,[[]]],[11,"into","","",27,[[]]],[11,"to_owned","","",27,[[]]],[11,"clone_into","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"try_into","","",27,[[],["result",4]]],[11,"borrow","","",27,[[]]],[11,"borrow_mut","","",27,[[]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"vzip","","",27,[[]]],[11,"from","sptir::spectrum::xyz","",28,[[]]],[11,"into","","",28,[[]]],[11,"to_owned","","",28,[[]]],[11,"clone_into","","",28,[[]]],[11,"try_from","","",28,[[],["result",4]]],[11,"try_into","","",28,[[],["result",4]]],[11,"borrow","","",28,[[]]],[11,"borrow_mut","","",28,[[]]],[11,"type_id","","",28,[[],["typeid",3]]],[11,"vzip","","",28,[[]]],[11,"from","sptir::world::aggregate","",32,[[]]],[11,"into","","",32,[[]]],[11,"try_from","","",32,[[],["result",4]]],[11,"try_into","","",32,[[],["result",4]]],[11,"borrow","","",32,[[]]],[11,"borrow_mut","","",32,[[]]],[11,"type_id","","",32,[[],["typeid",3]]],[11,"vzip","","",32,[[]]],[11,"set_sample","sptir::sampling","",18,[[]]],[11,"set_pixel","","",18,[[]]],[11,"next1d","","",18,[[["sampledimension1d",4]],["sample1d",3]]],[11,"next2d","","",18,[[["sampledimension2d",4]],["sample2d",3]]],[11,"radiance","sptir::spectrum::distributions::illuminant_d","",19,[[["spectralwavelengths",3]],["spectralradiance",3]]],[11,"radiance","sptir::spectrum::distributions::sampled","",20,[[["spectralwavelengths",3]],["spectralradiance",3]]],[11,"reflectance","","",20,[[["spectralwavelengths",3]],["spectralreflectance",3]]],[11,"clone","sptir::color","",0,[[],["colormatrix3",3]]],[11,"clone","","",1,[[],["xyy",3]]],[11,"clone","","",2,[[],["rgbcolor",3]]],[11,"clone","","",3,[[],["rgbprimaries",3]]],[11,"clone","sptir::geometry","",8,[[],["matrix4",3]]],[11,"clone","","",9,[[],["point3",3]]],[11,"clone","","",10,[[],["ray",3]]],[11,"clone","","",11,[[],["transform3",3]]],[11,"clone","","",12,[[],["vector3",3]]],[11,"clone","sptir::sampling","",13,[[],["sample1d",3]]],[11,"clone","","",14,[[],["sample2d",3]]],[11,"clone","","",15,[[],["sampledimension1d",4]]],[11,"clone","","",16,[[],["sampledimension2d",4]]],[11,"clone","sptir::spectrum::distributions::illuminant_d","",19,[[],["dseries",3]]],[11,"clone","sptir::spectrum::range","",23,[[],["spectralrange",3]]],[11,"clone","sptir::spectrum::units","",24,[[],["spectralwavelengths",3]]],[11,"clone","","",27,[[],["spectralflux",3]]],[11,"clone","sptir::spectrum::xyz","",28,[[],["xyzspectrum",3]]],[11,"default","sptir::geometry","",8,[[]]],[11,"default","","",11,[[],["transform3",3]]],[11,"eq","sptir::color","",0,[[["colormatrix3",3]]]],[11,"ne","","",0,[[["colormatrix3",3]]]],[11,"eq","","",1,[[["xyy",3]]]],[11,"ne","","",1,[[["xyy",3]]]],[11,"eq","","",2,[[["rgbcolor",3]]]],[11,"ne","","",2,[[["rgbcolor",3]]]],[11,"eq","","",3,[[["rgbprimaries",3]]]],[11,"ne","","",3,[[["rgbprimaries",3]]]],[11,"eq","sptir::geometry","",8,[[["matrix4",3]]]],[11,"ne","","",8,[[["matrix4",3]]]],[11,"eq","","",9,[[["point3",3]]]],[11,"ne","","",9,[[["point3",3]]]],[11,"eq","","",10,[[["ray",3]]]],[11,"ne","","",10,[[["ray",3]]]],[11,"eq","","",11,[[["transform3",3]]]],[11,"ne","","",11,[[["transform3",3]]]],[11,"eq","","",12,[[["vector3",3]]]],[11,"ne","","",12,[[["vector3",3]]]],[11,"eq","sptir::sampling","",13,[[["sample1d",3]]]],[11,"ne","","",13,[[["sample1d",3]]]],[11,"eq","","",14,[[["sample2d",3]]]],[11,"ne","","",14,[[["sample2d",3]]]],[11,"eq","","",15,[[["sampledimension1d",4]]]],[11,"eq","","",16,[[["sampledimension2d",4]]]],[11,"eq","sptir::spectrum::distributions::illuminant_d","",19,[[["dseries",3]]]],[11,"ne","","",19,[[["dseries",3]]]],[11,"eq","sptir::spectrum::range","",23,[[["spectralrange",3]]]],[11,"ne","","",23,[[["spectralrange",3]]]],[11,"eq","sptir::spectrum::units","",24,[[["spectralwavelengths",3]]]],[11,"ne","","",24,[[["spectralwavelengths",3]]]],[11,"eq","","",27,[[["spectralflux",3]]]],[11,"ne","","",27,[[["spectralflux",3]]]],[11,"eq","sptir::spectrum::xyz","",28,[[["xyzspectrum",3]]]],[11,"ne","","",28,[[["xyzspectrum",3]]]],[11,"fmt","sptir::color","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::geometry","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::sampling","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","","",16,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::spectrum::distributions::illuminant_d","",19,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::spectrum::range","",23,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::spectrum::units","",24,[[["formatter",3]],["result",6]]],[11,"fmt","","",27,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::spectrum::xyz","",28,[[["formatter",3]],["result",6]]],[11,"div","sptir::color","",2,[[]]],[11,"div","sptir::geometry","",11,[[["transform3",3]],["transform3",3]]],[11,"div","","",12,[[],["vector3",3]]],[11,"sub","","",8,[[],["matrix4",3]]],[11,"sub","","",9,[[["point3",3]],["vector3",3]]],[11,"sub","","",9,[[["vector3",3]],["point3",3]]],[11,"sub","","",12,[[],["vector3",3]]],[11,"add","sptir::color","",2,[[]]],[11,"add","sptir::geometry","",8,[[],["matrix4",3]]],[11,"add","","",9,[[["vector3",3]],["point3",3]]],[11,"add","","",12,[[],["vector3",3]]],[11,"add","sptir::spectrum::units","",25,[[],["spectralradiance",3]]],[11,"add","","",27,[[],["spectralflux",3]]],[11,"add","sptir::spectrum::xyz","",28,[[["xyzspectrum",3]],["xyzspectrum",3]]],[11,"mul","sptir::color","",2,[[]]],[11,"mul","","",2,[[]]],[11,"mul","sptir::geometry","",8,[[],["matrix4",3]]],[11,"mul","","",8,[[],["matrix4",3]]],[11,"mul","","",8,[[["point3",3]]]],[11,"mul","","",8,[[["vector3",3]]]],[11,"mul","","",11,[[],["transform3",3]]],[11,"mul","","",12,[[],["vector3",3]]],[11,"mul","sptir::spectrum::units","",25,[[],["spectralflux",3]]],[11,"mul","","",26,[[],["spectralreflectance",3]]],[11,"mul","","",27,[[["spectralreflectance",3]],["spectralflux",3]]],[11,"mul","","",27,[[],["spectralflux",3]]]],"p":[[3,"ColorMatrix3"],[3,"xyY"],[3,"RGBColor"],[3,"RGBPrimaries"],[6,"SRGBColor"],[8,"Colorspace"],[3,"Adaptation"],[3,"Film"],[3,"Matrix4"],[3,"Point3"],[3,"Ray"],[3,"Transform3"],[3,"Vector3"],[3,"Sample1D"],[3,"Sample2D"],[4,"SampleDimension1D"],[4,"SampleDimension2D"],[8,"Sampler"],[3,"IndependentSampler"],[3,"DSeries"],[3,"PiecewiseLinearSpectrum"],[8,"SpectralPowerDistribution"],[8,"SpectralReflectanceDistribution"],[3,"SpectralRange"],[3,"SpectralWavelengths"],[3,"SpectralRadiance"],[3,"SpectralReflectance"],[3,"SpectralFlux"],[3,"XYZSpectrum"],[3,"ACES_AP0"],[3,"ACES_AP1"],[3,"SRGB"],[4,"Aggregate"]]},\
"vis":{"doc":"","i":[[5,"main","vis","",null,[[]]]],"p":[]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);