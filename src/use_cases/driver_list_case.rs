use crate::domain::api_caller::ApiCaller;
use crate::domain::flow_input::FlowInput;
use crate::domain::path_generator::PathGenerator;
use crate::domain::user::User;
use futures::stream::{self, StreamExt};
use reqwest::Response;

pub struct DriverListCase {
    flow_input: FlowInput,
}

impl DriverListCase {
    // per ora instanziamo le dipendenze dentro il metodo
    // in futuro le passeremo come parametri per dependency injection
    pub fn new(config_file_path: String) -> Self {
        let absolute_config_path = PathGenerator::from_string(&config_file_path);
        let flow_input = FlowInput::from_json_file(absolute_config_path);
        DriverListCase { flow_input }
    }
    pub async fn execute(&self) {
        let _ = stream::iter(self.flow_input.get_users())
            .map(|user| self.perform_flow_for_user(user))
            .buffer_unordered(10) // numero di task concorrenti
            .collect::<Vec<_>>()
            .await;
    }

    async fn perform_flow_for_user(&self, user: &User) -> Response {
        let login_res = self.perform_login(user);
        let responses = self.perform_driver_list_call(login_res, user).await;
        print!("Response for user {}: {:?}", user.email, responses);
        responses
    }
    
    fn perform_driver_list_call(&self, response: impl Future<Output=Response>, user: &User) -> impl Future<Output=Response> {
        async move {
            let token = response.await;
            let login_step = self.flow_input.get_step_by_name("login").expect("Login step not found");
            let api_caller = ApiCaller::from_step(login_step, user);
            api_caller.post_request().await
        }
    }

    // serve l async move per catturare api_caller, poiché viene creato dentro la funzione e il risultato
    // è legato alla vita di api_caller. Quindi col blocco async spostiamo tutto in una 'bolla' 
    // che tiene tutte le variabili. Questo è necessario in rust perchè le closure devono essere indipendenti
    // dal contesto esterno, e non possono fare riferimento a variabili che potrebbero non esistere più quando la closure viene eseguita.
    fn perform_login<'a>(&'a self, user: &'a User) -> impl Future<Output=Response> + 'a {
        async move {
            let login_step = self.flow_input.get_step_by_name("login").expect("Login step not found");
            let api_caller = ApiCaller::from_step(login_step, user);
            api_caller.get_request().await
        }
    }
}
 