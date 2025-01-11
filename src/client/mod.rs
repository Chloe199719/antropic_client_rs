use serde::{Deserialize, Serialize};

pub struct AnthropicClient {
    api_key: String,
    api_url: String,
    client: reqwest::Client,
}
impl AnthropicClient {
    pub fn new(api_key: String, api_url: String) -> Self {
        Self {
            api_key,
            api_url,
            client: reqwest::Client::new(),
        }
    }
    pub async fn get_message_completed(
        &self,
        body: RequestBodyAnthropic,
    ) -> Result<ResponseBodyAnthropic, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/v1/messages", self.api_url))
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {}
            _ => {
                return Err(anyhow::anyhow!(
                    "Error: {}",
                    res.text().await.unwrap_or("".to_string())
                ));
            }
        }
        // .text()
        let body = res.json::<ResponseBodyAnthropic>().await?;
        Ok(body)
    }
}
#[derive(Debug, Serialize, Deserialize)]
/// Request body for the Anthropic API
///   model: The model to use for the completion
///  max_tokens: The maximum number of tokens to generate
///  messages: The messages to use for the completion
/// temperature: The temperature to use for the completion
///
pub struct RequestBodyAnthropic {
    pub model: String,
    pub max_tokens: i32,
    pub messages: Vec<Messages>,
    pub temperature: Option<f32>,
}
impl Default for RequestBodyAnthropic {
    fn default() -> Self {
        Self {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 1000,
            messages: vec![],
            temperature: Some(0.1),
        }
    }
}
impl RequestBodyAnthropic {
    pub fn new(
        model: String,
        max_tokens: i32,
        messages: Vec<Messages>,
        temperature: Option<f32>,
    ) -> Self {
        Self {
            model,
            max_tokens,
            messages,
            temperature,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    String(String),
    ContentArray(Vec<ContentType>),
}
/// Messages to be sent to the API
/// role: The role of the message
/// content: The content of the message
#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub role: Role,
    pub content: MessageContent,
}
impl Messages {
    pub fn new(role: Role, content: MessageContent) -> Self {
        Self { role, content }
    }
    /// Create a new message prompt
    /// content: The content of the message
    pub fn new_user_message_prompt(content: String) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::String(content),
        }
    }
    /// Create a new assistant message prompt
    /// content: The content of the message
    pub fn new_assistant_message_prompt(content: String) -> Self {
        Self {
            role: Role::Assistant,
            content: MessageContent::String(content),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}
impl Role {
    pub fn new(role: &str) -> Self {
        match role {
            "user" => Self::User,
            "assistant" => Self::Assistant,
            _ => Self::User,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ResponseBodyAnthropic {
    pub id: String,
    pub model: String,
    pub role: Role,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub usage: Usage,
    pub content: Vec<ContentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub data: Option<String>,
    pub media_type: Option<MediaType>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentText {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentImage {
    pub source: Source,
    #[serde(rename = "type")]
    pub content_type: String,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct Source {
    #[serde(rename = "type")]
    pub content_type: String,
    pub data: String,
    pub media_type: MediaType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/gif")]
    Gif,
    #[serde(rename = "image/webp")]
    Webp,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]

pub enum ContentType {
    #[serde(rename = "text")]
    Text(ContentText),
    #[serde(rename = "image")]
    Image(ContentImage),
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::STANDARD, Engine};

    #[tokio::test]
    async fn test_get_message_completed() {
        dotenvy::dotenv().ok();
        let client = AnthropicClient::new(
            std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not found"),
            "https://api.anthropic.com".to_string(),
        );
        let messages = vec![Messages {
            role: Role::User,
            content: MessageContent::String("What is the capital of France?".to_string()),
        }];
        let body = RequestBodyAnthropic {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 1000,
            messages,
            temperature: Some(0.1),
        };
        match client.get_message_completed(body).await {
            Ok(res) => {
                // assert_eq!(res.role, Role::Assistant);
                println!("{:#?}", res);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[tokio::test]
    async fn test_string_message() {
        dotenvy::dotenv().ok();
        let client = AnthropicClient::new(
            std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not found"),
            "https://api.anthropic.com".to_string(),
        );
        let messages = vec![Messages {
            role: Role::User,
            content: MessageContent::String("What is the capital of France?".to_string()),
        }];
        let body = RequestBodyAnthropic {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 1000,
            messages,
            temperature: Some(0.1),
        };
        match client.get_message_completed(body).await {
            Ok(res) => {
                // assert_eq!(res.role, Role::Assistant);
                println!("{:#?}", res);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_content_array_message() {
        dotenvy::dotenv().ok();
        let client = AnthropicClient::new(
            std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not found"),
            "https://api.anthropic.com".to_string(),
        );

        let image_bytes = reqwest::get("https://rocketutor-math.s3.eu-central-1.amazonaws.com/ocr/GHuO0CD28Ut8eBMxQwgjD5bNFfCp/solution4_boris.jpg")
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        let image_base64 = STANDARD.encode(image_bytes);

        let content = vec![
            ContentType::Text(ContentText {
                text: r#"### Identity: A maths teacher reviewing the math homework of students. 
Identify and highlight calculation errors and errors in the drawing of geometries in math assignments for grades 7 to 12. 
Goal:
Review math assignments from grades 7 to 12 by comparing the student's solutions (provided in LaTeX text format) with the correct solutions in our system. Determine the correctness of the student's solutions and provide detailed explanations highlighting any errors found in calculations or geometric drawings. Only count the solution as wrong when its mathematically wrong. If its just arranged differently then you shouldnt consider it wrong. point out exactly where the errors are and cite the wrong part, afterwards give the corrected version of that part
# Steps
1. Review Each Problem and Solution:
   - Carefully read each problem and its corresponding solution provided in our system.
   - Ensure a clear understanding of the mathematical concepts and operations involved.
2. Review User's Solution in LaTeX Format:
   - Examine the student's solution provided in LaTeX text format.
   - Verify if the angles and geometric constructions match those in the provided solution.
   - Check the mathematical formulas used and confirm their correctness based on the provided solutions.
3. Perform Independent Calculations:
   - Note: Certain assignments may have some error tolerance in the final result, especially when π is involved.
   - Independently perform the calculations presented in the student's solution.
   - Use standard arithmetic operations to arrive at the answer.
4. Compare and Identify Errors:
   - Compare the student's solution with the correct solution from our system.
   - Identify any calculation errors or inaccuracies in the geometric drawings.
   - Note any discrepancies in formulas, calculations, or geometric constructions.
5. Provide Feedback:
   - Determine if the student's solution is correct.
   - Prepare a message explaining why the solution is correct or incorrect, highlighting specific errors. You should also shortly explain the mathematical rules that need to be used in order to get to the correct result
# Input Format
Question: <German Assignment text>
System_Solution: <German Correct solution>
User_solution: <German Students solution to check>
# Output Format
Provide your assessment in the following JSON format:
json
{
  "correct": <true or false>,
  "messages": "<english Explanation of why the solution is correct or incorrect directly to the student>"
}

# Examples
Example 1:
- Input: A student's solution of a geometry problem provided in LaTeX.
- Output: 
  json
  {
    "correct": false,
    "messages": "You calculated the gemotetry problem incorrect. try doing .. differently."
  }
  
Example 2:
- Input: A student's solution of an algebraic calculation provided in LaTeX.
- Output: 
  json
  {
    "correct": true,
    "messages": "Your solution is correct."
  }
  
# Notes
- The students solution doesnt need to match exactly with the provided system_solution, often it has different intermediate calculations. as long as the final result is mathematically the same consider the calculation as correct. note, these terms are equivalent "4-1" and "-1+4"
- Pay particular attention to the subjectivity in geometric interpretations if the instructions leave some room for creative construction.
- Ensure precision and clarity to avoid any misunderstanding, particularly in error explanations."#.to_string(),
                content_type: "text".to_string(),
            }),
            ContentType::Text(ContentText {
                text: r#"Assignment: Bestimme die Ableitung <math>f^\\prime(x)</math> für <math>f(x)=\\frac{1}{x^5}</math> mit der Potenzregel für Ableitungen.\n    /n System Solution: <p><strong>(Schritt 1) Berechnen der Ableitung &lt;math&gt;f^\\prime(x)&lt;/math&gt;</strong></p>\n<p>&lt;KE id=\"nJABy-dovv1_ZzeHb2MpYgfgTq_s\"&gt; Die Potenzregel für Ableitungen besagt: Für &lt;math&gt;f(x)=x^n&lt;/math&gt; (&lt;math&gt;n \\in \\mathbb{R}&lt;/math&gt; mit &lt;math&gt;n\\neq 0&lt;/math&gt;) gilt &lt;math&gt;f^\\prime(x)=n\\cdot x^{n-1}&lt;/math&gt;.&lt;/KE&gt;</p>\n<p>  </p>\n<p>Um die Potenzregel für Ableitungen verwenden zu können, wandeln wir den Bruch &lt;math&gt;f(x)=\\frac{1}{x^5}&lt;/math&gt; zunächst in eine Potenz um:</p>\n<p>&lt;math&gt;f(x)=\\frac{1}{x^5}&lt;/math&gt;&lt;KE id=\"abUTiDUaheWEjVqypPYzCjN8cHgc\"&gt;&lt;math&gt;\\\\ | \\\\ x^{-n}= \\frac{1}{x^n}&lt;/math&gt; &lt;/KE&gt;</p>\n<p>&lt;math&gt;f(x)=x^{-5}&lt;/math&gt;</p>\n<p>Nun können wir mit der Potenzregel die Ableitung &lt;math&gt;f^\\prime(x)&lt;/math&gt; bestimmen:</p>\n<p>&lt;math&gt;f(x)=x^{-5}&lt;/math&gt;&lt;KE id=\"nJABy-dovv1_ZzeHb2MpYgfgTq_s\"&gt; &lt;math&gt;\\\\ | \\\\ f(x)=x^n \\to f^\\prime(x) = n\\cdot x^{n-1}&lt;/math&gt;&lt;/KE&gt;</p>\n<p>&lt;math&gt;f^\\prime(x)=-5\\cdot x^{-5-1}&lt;/math&gt;</p>\n<p>&lt;math&gt;f^\\prime(x)=-5\\cdot x^{-6}&lt;/math&gt;&lt;KE id=\"abUTiDUaheWEjVqypPYzCjN8cHgc\"&gt;&lt;math&gt;\\\\ | \\\\ x^{-n}= \\frac{1}{x^n}&lt;/math&gt; &lt;/KE&gt;</p>\n<p>&lt;math&gt;f^\\prime(x)=\\frac{-5}{x^{6}} &lt;/math&gt;</p>\n<p>  </p>\n<p><strong>Antwort: Die Ableitung von &lt;math&gt;f(x)=\\frac{1}{x^5}&lt;/math&gt; lautet &lt;math&gt;f^\\prime(x) = \\frac{-5}{x^{6}}&lt;/math&gt;.</strong></p>\n\n    /n  student_solution: \n    \\( f^{\\prime} \\) for \\( f(x)=\\frac{1}{x^{5}} \\) bastirnmen \\[ \\begin{array}{l} f(x)=\\frac{1}{x^{5}}=x^{-5} \\\\ f^{\\prime}(x)=-5 \\cdot x^{-6}=-\\frac{5}{x^{6}} \\end{array} \\]\n\n\n        "#.to_string(),
                content_type: "text".to_string(),
            }),
            ContentType::Image(ContentImage {
               source: Source {
                content_type: "base64".to_string(),
                data: image_base64,
                media_type: MediaType::Jpeg,
               },
               content_type: "image".to_string(),
            })
        ];
        let messages = vec![Messages {
            role: Role::User,
            content: MessageContent::ContentArray(content),
        }];
        let body = RequestBodyAnthropic {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 1000,
            messages,
            temperature: Some(0.1),
        };
        match client.get_message_completed(body).await {
            Ok(res) => {
                // assert_eq!(res.role, Role::Assistant);
                println!("{:#?}", res);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
