use diesel::PgConnection;
use diesel::prelude::*;
use crate::model::Infra;
use crate::schema::infra::dsl::*;
use crate::schema::infra;



pub fn create( infra_object: Infra, conn: &mut PgConnection ) -> Option<Infra> {

    let result: Result<Infra, diesel::result::Error> = diesel::insert_into(infra::table).values(&infra_object).get_result(conn);//.expect("Error saving new post");

    match result {
        Ok(infra_object) => Some(infra_object),
        Err(error) => {
            println!( "Error creating new Infra object: {error}" );  
            None
        }
    }

}

pub fn get_one( infra_name: &str, conn: &mut PgConnection ) -> Option<Infra> {

    let result: Result<Infra, diesel::result::Error> = infra.find(infra_name).first(conn);

    match result {
        Ok(infra_object) => Some(infra_object),
        Err(error) => {
            println!( "Error: {error} ---> can't find {infra_name}" );
            None
        }
    }
}

pub fn get_all( conn: &mut PgConnection ) -> Option<Vec<Infra>> {

    let results: Vec<Infra> = infra.load( conn ).expect("Error loading Infra objects");

    if results.len() != 0 {
        Some(results)
    } else {
        None
    }
}

pub fn update( infra_name: &str, infra_modifier_param: Option<f64>, price_param: Option<i32>, conn: &mut PgConnection ) -> Option<Infra> {
    
    match &mut get_one(infra_name, conn) {

        // if object found
        Some(infra_object) => {

            // update infra modifier value if provided
            match infra_modifier_param {
                Some(value) => {
                    infra_object.infra_modifier = Some(value);
                    // let result = diesel::update(crate::schema::infra::table).filter(name.eq(infra_name)).set(infra::infra_modifier.eq(value)).execute(conn);
                    ()
                },
                None => ()
            }
            
            // update price value if provided
            match price_param {
                Some(value) => {
                    infra_object.price = Some(value);
                    // let result = diesel::update(crate::schema::infra::table).filter(name.eq(infra_name)).set(infra::price.eq(value)).execute(conn);
                    ()
                },
                None => ()
            }

            // updates with all changes applied - if any
            let _ = diesel::update(infra::table).set(infra_object.clone() ).execute(conn);

            Some( infra_object.clone() )
        },

        // object not found
        None => {
            println!("Can't update object that doesn't exists: {infra_name}");
            None
        }
    }

    
}

pub fn delete( infra_name: &str, conn: &mut PgConnection ) -> Option<()> {

    match &mut get_one(infra_name, conn) {

        // if object found
        Some(_) => {
            let _ = diesel::delete(infra::table).filter(name.eq(infra_name)).execute(conn);
            Some( () )
        },

        // object not found
        None => {
            println!("Can't delete object that doesn't exists: {infra_name}");
            None
        }
    }
}