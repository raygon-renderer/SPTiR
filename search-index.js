var searchIndex = JSON.parse('{\
"sptir":{"doc":"","i":[[5,"main","sptir","",null,[[]]],[0,"geometry","","",null,null],[3,"Matrix4","sptir::geometry","A 4x4 matrix",null,null],[12,"m00","","",0,null],[12,"m01","","",0,null],[12,"m02","","",0,null],[12,"m03","","",0,null],[12,"m10","","",0,null],[12,"m11","","",0,null],[12,"m12","","",0,null],[12,"m13","","",0,null],[12,"m20","","",0,null],[12,"m21","","",0,null],[12,"m22","","",0,null],[12,"m23","","",0,null],[12,"m30","","",0,null],[12,"m31","","",0,null],[12,"m32","","",0,null],[12,"m33","","",0,null],[3,"Vector3","","",null,null],[12,"x","","",1,null],[12,"y","","",1,null],[12,"z","","",1,null],[18,"IDENTITY","","The Identity matrix",0,null],[11,"new","","",0,[[],["matrix4",3]]],[18,"ZERO","","",1,null],[18,"UP","","",1,null],[11,"new","","",1,[[],["vector3",3]]],[11,"dot","","The dot product (or scalar product) of two N-dimensional…",1,[[]]],[11,"cross","","The cross product (or vector product) is a binary…",1,[[],["vector3",3]]],[11,"norm_squared","","",1,[[]]],[11,"norm","","Compute the Euclidean norm (`$\\\\|\\\\mathbf{v}\\\\|$`) of the…",1,[[]]],[11,"normalize","","Normalizes the vector such that the 3D Euclidean norm is 1.0",1,[[],["vector3",3]]],[0,"math","sptir","",null,null],[5,"integrate","sptir::math","",null,[[]]],[0,"spectrum","sptir","",null,null],[3,"HeroWavelengthSample","sptir::spectrum","Hero-Wavelength Spectrum Sample",null,null],[12,"lambda","","The sampled wavelengths (`$\\\\lambda$`)",2,null],[12,"energy","","The Radiant Flux (`$\\\\Phi_{\\\\mathrm{e}}$`) being carried at…",2,null],[3,"XYZSpectrum","","",null,null],[12,"x","","",3,null],[12,"y","","",3,null],[12,"z","","",3,null],[3,"SpectralRange","","",null,null],[12,"min","","",4,null],[12,"max","","",4,null],[12,"y_integral","","",4,null],[6,"Lanes","","Helper type to define how many wavelength samples are…",null,null],[17,"NUM_LANES","","Defines how many wavelengths should be used for HWSS",null,null],[18,"ZERO","","",3,null],[11,"new","","",3,[[],["xyzspectrum",3]]],[11,"from_wavelength","","",3,[[],["xyzspectrum",3]]],[11,"new","","",4,[[],["spectralrange",3]]],[11,"hero_to_xyz","","`math X=\\\\sum_{j=0}^{C} \\\\left({\\\\frac {1}{N}}\\\\int _{\\\\lambda…",4,[[["herowavelengthsample",3]],["xyzspectrum",3]]],[11,"sample_hero","","Samples a hero wavelength and `NUM_LANES` number of…",4,[[],["herowavelengthsample",3]]],[11,"hero","","",2,[[]]],[11,"from","sptir::geometry","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","sptir::spectrum","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"clone","sptir::geometry","",0,[[],["matrix4",3]]],[11,"clone","","",1,[[],["vector3",3]]],[11,"clone","sptir::spectrum","",2,[[],["herowavelengthsample",3]]],[11,"clone","","",3,[[],["xyzspectrum",3]]],[11,"clone","","",4,[[],["spectralrange",3]]],[11,"eq","sptir::geometry","",0,[[["matrix4",3]]]],[11,"ne","","",0,[[["matrix4",3]]]],[11,"eq","","",1,[[["vector3",3]]]],[11,"ne","","",1,[[["vector3",3]]]],[11,"eq","sptir::spectrum","",2,[[["herowavelengthsample",3]]]],[11,"ne","","",2,[[["herowavelengthsample",3]]]],[11,"eq","","",3,[[["xyzspectrum",3]]]],[11,"ne","","",3,[[["xyzspectrum",3]]]],[11,"eq","","",4,[[["spectralrange",3]]]],[11,"ne","","",4,[[["spectralrange",3]]]],[11,"fmt","sptir::geometry","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","sptir::spectrum","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"div","sptir::geometry","",1,[[],["vector3",3]]]],"p":[[3,"Matrix4"],[3,"Vector3"],[3,"HeroWavelengthSample"],[3,"XYZSpectrum"],[3,"SpectralRange"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);