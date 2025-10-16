use tokio::time::{sleep, Duration};

use crate::domain::context::Context;
use crate::domain::api_caller::ApiCaller;
use crate::domain::flow_input::{FlowInput};
use crate::domain::step::{Step};
use crate::domain::path_generator::PathGenerator;
use crate::domain::user::User;
use futures::stream::{self, StreamExt};
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
    pub async fn execute(&self) {
        let now = std::time::Instant::now();
        let rampup = self.flow_input.get_rampup();
        stream::iter(self.flow_input.get_users().into_iter().enumerate())
           .map(|(i,u)| self.organize_rampup(rampup, i, u))
           .buffer_unordered(self.flow_input.get_threads())
           .collect::<Vec<_>>()
           .await;
       println!("Total time elapsed: {:?}", now.elapsed());
    }

    fn organize_rampup(&self, rampup: usize, i: usize, u: &User) -> impl Future<Output=ResponseWrapper> {
        let delay = Duration::from_millis(rampup as u64 * i as u64);
        async move {
            sleep(delay).await;
            self.perform_flow(u).await
        }
    }
    // per eseguire questi step in parallelo, usiamo stream::iter per creare uno stream di
    // futures da eseguire dopo e il valore di ritorno sara -> impl Stream<Item = impl Future<Output = Response> + 'a> + 'a
    // ma dopo andranno iterati perchè ritorna una lista, e non verranno eseguiti in serie.

    // per edeguire questi step in sequenza, dobbiamo awaitare ogni future prima di passare al successivo
    async fn perform_flow(&self, user: &User) -> ResponseWrapper {
        let mut responses = ResponseWrapper::from_email(user.email.clone());
        let mut context: Context = Context::new();
        for s  in self.flow_input.get_steps().iter() {
            let response = self.perform_step(user, &mut context, s).await;
            let body = response.text().await.unwrap_or_default();
            context.update(s, &body);
            responses.add(body);
        }
        responses
    }

    async fn perform_step(&self, user: &User, context: &mut Context, s: &Step) -> Response {
        println!("Performing step: {} for user: {}", s.action, user.email);
        let api_caller = ApiCaller::call(s, context, user);
        let response = api_caller.await;
        response
    }

    // serve l async move per catturare api_caller, poiché viene creato dentro la funzione e il risultato
    // è legato alla vita di api_caller. Quindi col blocco async spostiamo tutto in una 'bolla' 
    // che tiene tutte le variabili. Questo è necessario in rust perchè le closure devono essere indipendenti
    // dal contesto esterno, e non possono fare riferimento a variabili che potrebbero non esistere più quando la closure viene eseguita.
    
}
 