use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};
use gloo_utils::format::JsValueSerdeExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum WorkerMessage {
    #[serde(rename = "run")]
    Run { code: String },
    #[serde(rename = "stop")]
    Stop,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum WorkerResponse {
    #[serde(rename = "output")]
    Output { text: String },
    #[serde(rename = "error")]
    Error { text: String },
    #[serde(rename = "done")]
    Done,
}

pub struct PythonRunner {
    worker: Option<Worker>,
    on_message_closure: Option<Closure<dyn FnMut(MessageEvent)>>,
    on_error_closure: Option<Closure<dyn FnMut(MessageEvent)>>,
}

impl PythonRunner {
    pub fn new<F>(callback: F) -> Result<Self, JsValue>
    where
        F: Fn(WorkerResponse) + 'static,
    {
        let worker_code = include_str!("../static/worker.js");

        let blob_options = BlobPropertyBag::new();
        blob_options.set_type("application/javascript");
        let blob = Blob::new_with_str_sequence_and_options(
            &js_sys::Array::of1(&JsValue::from_str(worker_code)),
            &blob_options,
        )?;

        let url = Url::create_object_url_with_blob(&blob)?;
        let worker = Worker::new(&url)?;
        Url::revoke_object_url(&url)?;

        let callback = std::rc::Rc::new(callback);
        let callback_clone = callback.clone();

        let on_message_closure = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(data) = event.data().into_serde::<WorkerResponse>() {
                callback(data);
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let on_error_closure = Closure::wrap(Box::new(move |_event: MessageEvent| {
            callback_clone(WorkerResponse::Error {
                text: "Worker error occurred".to_string(),
            });
        }) as Box<dyn FnMut(MessageEvent)>);

        worker.set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        worker.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));

        Ok(PythonRunner {
            worker: Some(worker),
            on_message_closure: Some(on_message_closure),
            on_error_closure: Some(on_error_closure),
        })
    }

    pub fn run_code(&self, code: String) -> Result<(), JsValue> {
        if let Some(worker) = &self.worker {
            let message = WorkerMessage::Run { code };
            let js_message = serde_wasm_bindgen::to_value(&message)?;
            worker.post_message(&js_message)?;
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<(), JsValue> {
        if let Some(worker) = &self.worker {
            let message = WorkerMessage::Stop;
            let js_message = serde_wasm_bindgen::to_value(&message)?;
            worker.post_message(&js_message)?;
        }
        Ok(())
    }
}

impl Drop for PythonRunner {
    fn drop(&mut self) {
        if let Some(worker) = &self.worker {
            worker.terminate();
        }
    }
}
