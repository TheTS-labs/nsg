use crate::basic_search::BasicSearch;
use crate::brief_request::BriefRequest;
use crate::payload::Payload;
use crate::view_request::ViewRequest;
use crate::work_schedule::WorkSchedule;
use crate::Nsg;

/// Implementation for bridge methods between Portal and data pillar parsers
impl Nsg {
    pub async fn work_schedule(&self, date: chrono::NaiveDate) -> WorkSchedule {
        let payload = serde_urlencoded::to_string(Payload {
            action: "workschedule1",
            city:   "",
            data:   &date.format("%Y-%m-%d").to_string(),
        })
        .unwrap();

        let response = self
            .request(self.construct_headers(), payload, None)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        WorkSchedule::from(&response)
    }

    pub async fn brief_request(&self, internal_order_id: u32) -> BriefRequest {
        let text = self
            .request(
                self.construct_headers(),
                format!("action=briefRequest&id={internal_order_id}&hold=0"),
                None,
            )
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        BriefRequest::from(&text)
    }

    pub async fn view_request(&self, internal_order_id: u32) -> ViewRequest {
        let session_code_body = self
            .request(
                self.construct_headers(),
                "".to_string(),
                Some(&format!("/index.php?action=viewRequest&id={internal_order_id}")),
            )
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut session_code = None;

        for line in session_code_body.lines() {
            if line.contains("sessioncode:") {
                session_code = Some(line.trim().replace("sessioncode:", "").replace(['"', ','], ""));
                break;
            }
        }

        let response = self
            .request(
                self.construct_headers(),
                "".to_string(),
                Some(&format!("/session.php?code={}", session_code.unwrap())),
            )
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        ViewRequest::from(&response)
    }

    // TODO: Provide User data pillar
    // /// Fetches and parses the user
    // pub async fn get_user(&self) -> serde_json::Value {
    //     let text = self
    //         .request(self.construct_headers(),
    // String::from("actiondata=core_getUser"), None)         .await
    //         .unwrap()
    //         .text()
    //         .await
    //         .unwrap();

    //     serde_json::from_str(&text).unwrap()
    // }

    pub async fn basic_search(&self, search_text: &str) -> BasicSearch {
        let text = self
            .request(
                self.construct_headers(),
                format!("action=search_basic&searchtext={search_text}"),
                None,
            )
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        BasicSearch::from(&text)
    }
}
