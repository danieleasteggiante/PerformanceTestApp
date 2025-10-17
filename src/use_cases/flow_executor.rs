use rayon::prelude::*;
use std::time::Duration;
use std::thread;
use crate::domain::context::Context;
use crate::domain::api_caller::ApiCaller;
use crate::domain::flow_input::{FlowInput};
use crate::domain::path_generator::PathGenerator;
use crate::domain::user::User;
use reqwest::Response;
use crate::domain::response_wrapper::ResponseWrapper;

pub struct FlowExecutor {
    flow_input: FlowInput,
    response: Option<Response>,
}

impl FlowExecutor {
    // per ora instanziamo le dipendenze dentro il metodo
    // in futuro le passeremo come parametri per dependency injection
    pub fn new(config_file_path: String) -> Self {
        let absolute_config_path = PathGenerator::from_string(&config_file_path);
        let flow_input = FlowInput::from_json_file(absolute_config_path);
        FlowExecutor { flow_input, response: None }
    }
    pub fn execute(&self) {
        let users = self.flow_input.get_users();
        let rampup = self.flow_input.get_rampup();
        let now = std::time::Instant::now();

        let responses: Vec<ResponseWrapper> = users
            .par_iter()
            .enumerate()
            .map(|(i, u)| self.organize_rampup(rampup, i, u))
            .collect();
        println!("All flows executed. Collected {} responses.", responses.len());
        println!("Total time elapsed: {:?}", now.elapsed());
    }
    fn organize_rampup(&self, rampup: usize, i: usize, u: &User) -> ResponseWrapper {
        let time_per_user = rampup as u64 * i as u64;
        let delay = Duration::from_millis(time_per_user);
        thread::sleep(delay);
        println!("Running flow per user {} - index {}", u.email, i);
        self.perform_flow(u)
    }
    // per eseguire questi step in parallelo, usiamo stream::iter per creare uno stream di
    // futures da eseguire dopo e il valore di ritorno sara -> impl Stream<Item = impl Future<Output = Response> + 'a> + 'a
    // ma dopo andranno iterati perchè ritorna una lista, e non verranno eseguiti in serie.

    // per edeguire questi step in sequenza, dobbiamo awaitare ogni future prima di passare al successivo
    fn perform_flow(&self, user: &User) -> ResponseWrapper {
        let mut responses = ResponseWrapper::from_email(user.email.clone());
        let mut context = Context::new();
        for s  in self.flow_input.get_steps().iter() {
            let response = ApiCaller::call(s, &context, user);
            let body = response.text().unwrap_or_else(|_| "Failed to read response body".to_string());
            context.update(s, &body);
            responses.add(body);
            let delay = Duration::from_millis(self.flow_input.get_intra_action_delay() as u64);
            thread::sleep(delay);
            println!("Running step {} per user {} in thread {:?}", s.action,  user.email, thread::current().id());
        }
        responses
    }
    // serve l async move per catturare api_caller, poiché viene creato dentro la funzione e il risultato
    // è legato alla vita di api_caller. Quindi col blocco async spostiamo tutto in una 'bolla' 
    // che tiene tutte le variabili. Questo è necessario in rust perchè le closure devono essere indipendenti
    // dal contesto esterno, e non possono fare riferimento a variabili che potrebbero non esistere più quando la closure viene eseguita.
}
 