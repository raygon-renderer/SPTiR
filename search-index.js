var searchIndex = JSON.parse('{\
"main":{"doc":"","i":[[5,"main","main","",null,[[]]]],"p":[]},\
"sptir":{"doc":"Spectral Path Tracing in Rust","i":[[0,"color","sptir","",null,null],[3,"ColorMatrix3","sptir::color","",null,null],[12,"0","","",0,null],[3,"xyY","","A two-dimensional coordinate on the xy-chromaticity…",null,null],[12,"x","","",1,null],[12,"y","","",1,null],[12,"luminance","","",1,null],[3,"Adaptation","","Helper struct for Chromatic Adaptation, converting from…",null,null],[3,"RGBColor","","Color consisting of Red, Green and Blue components,…",null,null],[12,"r","","",2,null],[12,"g","","",2,null],[12,"b","","",2,null],[3,"RGBPrimaries","","Defines the coordinates in the xy-chromaticity diagram for…",null,null],[12,"r","","",3,null],[12,"g","","",3,null],[12,"b","","",3,null],[5,"compute_adaptation_matrix","","Computes a linear transform to adapt one whitepoint to…",null,[[["xyy",3]],["colormatrix3",3]]],[18,"IDENTITY","","",0,null],[11,"inverse","","",0,[[],["colormatrix3",3]]],[11,"multiply","","",0,[[["colormatrix3",3]],["colormatrix3",3]]],[11,"transform_xyz","","",0,[[["xyzspectrum",3]],["xyzspectrum",3]]],[11,"transform_rgb","","WARNING: This method does not actually calculate the color…",0,[[["rgbcolor",3],["colorspace",8]],[["rgbcolor",3],["colorspace",8]]]],[11,"transpose","","",0,[[],["colormatrix3",3]]],[0,"spaces","","",null,null],[3,"ACES_AP0","sptir::color::spaces","ACES 2065-1 Extreme Wide-Gamut Color Space",null,null],[3,"ACES_AP1","","ACEScg Wide-Gamut Color Space",null,null],[3,"SRGB","","Rec701 sRGB Color Space",null,null],[6,"SRGBColor","","",null,null],[6,"ACES2065_1Color","","",null,null],[6,"ACEScgColor","","",null,null],[11,"decode_transfer_function_from","sptir::color","Decodes an 8-bit encoded sRGB color.",2,[[],["srgbcolor",6]]],[11,"encode_transfer_function_to","","Encodes a linear sRGB color to an output slice of 8-bit…",2,[[]]],[11,"decode_transfer_function_from","sptir::color::spaces","Decodes an 8-bit encoded sRGB color.",4,[[],["srgbcolor",6]]],[11,"encode_transfer_function_to","","Encodes a linear sRGB color to an output slice of 8-bit…",4,[[]]],[0,"whitepoints","sptir::color","",null,null],[17,"D65","sptir::color::whitepoints","Noon Daylight: Television, sRGB color space",null,null],[17,"D60","","D60 has a CCT of approximately 6000K",null,null],[17,"ACES","","D60-like whitepoint designed for future-compatibility of…",null,null],[17,"FORWARD_BRADFORD_MATRIX","sptir::color","`math \\\\begin{bmatrix} +0.89510 & +0.26640 & -0.16140 \\\\\\\\…",null,null],[17,"INVERSE_BRADFORD_MATRIX","","Inverse of `FORWARD_BRADFORD_MATRIX`, computed automatically",null,null],[8,"Colorspace","","Defines a working space within the human chromaticity…",null,null],[18,"PRIMARIES","","Defines the coordinates in the xy-chromaticity diagram for…",5,null],[18,"WHITEPOINT","","Defines the standard illuminant for the color space",5,null],[18,"RGB_TO_XYZ","","Conversion matrix from Linear RGB to XYZ tristimulus values",5,null],[18,"XYZ_TO_RGB","","Conversion matrix from XYZ tristimulus values to Linear RGB",5,null],[11,"new","","",1,[[],["xyy",3]]],[11,"to_xyz","","Converts xyY to XYZ tristimulus values",1,[[],["xyzspectrum",3]]],[18,"XYZ_TO_XYZ","","",6,null],[18,"RGB_TO_RGB","","",6,null],[11,"new","","",2,[[],["rgbcolor",3]]],[11,"convert","","Convert the RGB value of one colorspace to an RGB value in…",2,[[],[["rgbcolor",3],["colorspace",8]]]],[11,"rgb_to_xyz","","Derives the linear transformation matrix…",3,[[["xyy",3]],["colormatrix3",3]]],[0,"film","sptir","",null,null],[3,"Film","sptir::film","",null,null],[12,"pixels","","",7,null],[12,"width","","",7,null],[12,"height","","",7,null],[0,"geometry","sptir","",null,null],[3,"Matrix4","sptir::geometry","A 4x4 matrix",null,null],[12,"0","","",8,null],[3,"Vector3","","",null,null],[12,"x","","",9,null],[12,"y","","",9,null],[12,"z","","",9,null],[18,"IDENTITY","","The Identity matrix",8,null],[11,"new","","",8,[[],["matrix4",3]]],[18,"ZERO","","",9,null],[18,"UP","","",9,null],[11,"new","","",9,[[],["vector3",3]]],[11,"dot","","The dot product (or scalar product) of two N-dimensional…",9,[[]]],[11,"cross","","The cross product (or vector product) is a binary…",9,[[],["vector3",3]]],[11,"norm_squared","","",9,[[]]],[11,"norm","","Compute the Euclidean norm (`$\\\\|\\\\mathbf{v}\\\\|$`) of the…",9,[[]]],[11,"normalize","","Normalizes the vector such that the 3D Euclidean norm is 1.0",9,[[],["vector3",3]]],[0,"math","sptir","",null,null],[5,"integrate","sptir::math","",null,[[]]],[0,"spectrum","sptir","",null,null],[3,"HeroWavelengthSample","sptir::spectrum","Hero-Wavelength Spectrum Sample",null,null],[12,"lambda","","The sampled wavelengths (`$\\\\lambda$`)",10,null],[12,"energy","","The Radiant Flux (`$\\\\Phi_{\\\\mathrm{e}}$`) being carried at…",10,null],[12,"pdf","","The sampling probability for this wavelength",10,null],[3,"XYZSpectrum","","",null,null],[12,"x","","",11,null],[12,"y","","",11,null],[12,"z","","",11,null],[3,"SpectralRange","","",null,null],[12,"min","","",12,null],[12,"max","","",12,null],[12,"y_integral","","",12,null],[6,"Lanes","","Helper type to define how many wavelength samples are…",null,null],[17,"NUM_LANES","","Defines how many wavelengths should be used for HWSS",null,null],[18,"ZERO","","",11,null],[11,"new","","",11,[[],["xyzspectrum",3]]],[11,"from_wavelength","","Calculates the tristimulus response values for the given…",11,[[],["xyzspectrum",3]]],[11,"new","","",12,[[],["spectralrange",3]]],[11,"hero_to_xyz","","",12,[[["herowavelengthsample",3]],["xyzspectrum",3]]],[11,"sample_hero","","Samples a hero wavelength and `NUM_LANES` number of…",12,[[],["herowavelengthsample",3]]],[11,"hero","","",10,[[]]],[11,"from","sptir::color","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","sptir::color::spaces","",13,[[]]],[11,"into","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","sptir::film","",7,[[]]],[11,"into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","sptir::geometry","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","sptir::spectrum","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"clone","sptir::color","",0,[[],["colormatrix3",3]]],[11,"clone","","",1,[[],["xyy",3]]],[11,"clone","","",2,[[],["rgbcolor",3]]],[11,"clone","","",3,[[],["rgbprimaries",3]]],[11,"clone","sptir::geometry","",8,[[],["matrix4",3]]],[11,"clone","","",9,[[],["vector3",3]]],[11,"clone","sptir::spectrum","",10,[[],["herowavelengthsample",3]]],[11,"clone","","",11,[[],["xyzspectrum",3]]],[11,"clone","","",12,[[],["spectralrange",3]]],[11,"eq","sptir::color","",0,[[["colormatrix3",3]]]],[11,"ne","","",0,[[["colormatrix3",3]]]],[11,"eq","","",1,[[["xyy",3]]]],[11,"ne","","",1,[[["xyy",3]]]],[11,"eq","","",2,[[["rgbcolor",3]]]],[11,"ne","","",2,[[["rgbcolor",3]]]],[11,"eq","","",3,[[["rgbprimaries",3]]]],[11,"ne","","",3,[[["rgbprimaries",3]]]],[11,"eq","sptir::geometry","",8,[[["matrix4",3]]]],[11,"ne","","",8,[[["matrix4",3]]]],[11,"eq","","",9,[[["vector3",3]]]],[11,"ne","","",9,[[["vector3",3]]]],[11,"eq","sptir::spectrum","",10,[[["herowavelengthsample",3]]]],[11,"ne","","",10,[[["herowavelengthsample",3]]]],[11,"eq","","",11,[[["xyzspectrum",3]]]],[11,"ne","","",11,[[["xyzspectrum",3]]]],[11,"eq","","",12,[[["spectralrange",3]]]],[11,"ne","","",12,[[["spectralrange",3]]]],[11,"fmt","sptir::color","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::geometry","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::spectrum","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"div","sptir::geometry","",9,[[],["vector3",3]]]],"p":[[3,"ColorMatrix3"],[3,"xyY"],[3,"RGBColor"],[3,"RGBPrimaries"],[6,"SRGBColor"],[8,"Colorspace"],[3,"Adaptation"],[3,"Film"],[3,"Matrix4"],[3,"Vector3"],[3,"HeroWavelengthSample"],[3,"XYZSpectrum"],[3,"SpectralRange"],[3,"ACES_AP0"],[3,"ACES_AP1"],[3,"SRGB"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);