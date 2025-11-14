use std::fs;

use gpkg::{GeoPackage, GPKGModel, SpatialRefSys};
use gpkg::types::GPKGPointZ;

fn main() {
    // Path for the GeoPackage
    let gpkg_path = "geopackage.gpkg";
    
    // Remove existing file if it exists
    if std::path::Path::new(gpkg_path).exists() {
        fs::remove_file(gpkg_path).expect("Failed to remove existing GeoPackage file");
    }
    
    // Create a new GeoPackage
    let mut gpkg = match GeoPackage::create(gpkg_path) {
        Ok(gpkg) => gpkg,
        Err(e) => {
            eprintln!("Failed to open GeoPackage: {}", e);
            return;
        }
    };

    // Add spatial reference system for UTM 32N
    let srs = SpatialRefSys {
        name: "ETRS 1989 / UTM zone 32N",
        id: 25832,
        organization: "EPSG",
        organization_coordsys_id: 25832,
        definition: "PROJCS[\"ETRS_1989_UTM_Zone_32N\",GEOGCS[\"GCS_ETRS_1989\",DATUM[\"D_ETRS_1989\",SPHEROID[\"GRS_1980\",6378137.0,298.257222101]],PRIMEM[\"Greenwich\",0.0],UNIT[\"Degree\",0.0174532925199433]],PROJECTION[\"Transverse_Mercator\"],PARAMETER[\"False_Easting\",500000.0],PARAMETER[\"False_Northing\",0.0],PARAMETER[\"Central_Meridian\",9.0],PARAMETER[\"Scale_Factor\",0.9996],PARAMETER[\"Latitude_Of_Origin\",0.0],UNIT[\"Meter\",1.0]]",
        description: "ETRS_1989_UTM_Zone_32N",
    };
    gpkg.new_srs(&srs).unwrap();

    // Define and create a new table layer
    #[derive(GPKGModel)]
    #[layer_name = "test_table"]
    struct TestTable {
        field1: i64,
        field2: String,
        field3: f64,
    }
    gpkg.create_layer::<TestTable>().unwrap();
    
    // Define and create a new point geometry layer
    #[derive(GPKGModel)]
    #[layer_name = "point_layer"]
    struct PointLayer {
        name: String,
        #[geom_field("PointZ")]
        geom: GPKGPointZ,
    }
    gpkg.create_layer::<PointLayer>().unwrap();
    gpkg.update_layer_srs_id("point_layer", srs.id).unwrap();

    // Add records to tesst_table
    let test_table_records = vec![
        TestTable { field1: 1, field2: "First".to_string(), field3: 1.1 },
        TestTable { field1: 2, field2: "Second".to_string(), field3: 2.2 },
        TestTable { field1: 3, field2: "Third".to_string(), field3: 3.3 },
    ];
    gpkg.insert_many(&test_table_records).unwrap();

    // Add records to point_layer
    let point_layer_records = vec![
        PointLayer { name: "Point A".to_string(), geom: GPKGPointZ{x: 400000.0, y: 5500000.0, z: 100.0} },
        PointLayer { name: "Point B".to_string(), geom: GPKGPointZ{x: 400100.0, y: 5500100.0, z: 150.0} },
        PointLayer { name: "Point C".to_string(), geom: GPKGPointZ{x: 400200.0, y: 5500200.0, z: 200.0} },
    ];
    gpkg.insert_many(&point_layer_records).unwrap();

    // Close the GeoPackage
    gpkg.close();

}
