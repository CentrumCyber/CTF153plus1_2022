import java.util.UUID;  

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.boot.configurationprocessor.json.JSONObject;
import org.springframework.boot.web.client.RestTemplateBuilder;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;

import org.springframework.web.bind.annotation.RequestParam;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.apache.logging.log4j.ThreadContext;

@RestController
@CrossOrigin
@RequestMapping(path="/api", produces = "application/json")
public class MainController {
    private static final Logger logger = LogManager.getLogger("HelloWorld");
    private final RestTemplate restTemplate;
    private String _apiKey = "<REDACTED>";

    @GetMapping("get-gif")
    public ResponseEntity<ResponseData> index(@RequestParam("input") String input) {
        ThreadContext.put("id", UUID.randomUUID().toString());
        ThreadContext.put("flag", "CTF{sample_flag}");

        String strippedInput = input
            .trim()
            .replace("jndi", "")
            .replace("ldap", "")
            .replace("lower", "")
            .replace("upper", "");

        logger.info("Request by uuid: {}", ThreadContext.get("id"));
        logger.info("Query for: {}", strippedInput);

        String url = "https://api.giphy.com/v1/gifs/search?api_key={key}&q={query}&limit=50&offset=0&rating=g&lang=en";
        ResponseEntity<String> response = restTemplate.getForEntity(url, String.class, _apiKey, strippedInput);

        String data = "https://media3.giphy.com/media/QARvzniFRIIyELxRm3/giphy.gif?cid=26cf736cn5tb5umlyamkb2tyn4dn8njg3ohxh14brolpgxtx&rid=giphy.gif&ct=g";
        try {
            data = (String)new JSONObject(response.getBody())
                .getJSONArray("data")
                .getJSONObject((int)(Math.random() * 50))
                .getJSONObject("images")
                .getJSONObject("original")
                .get("url");
            logger.info("Received data: {}", data);
        } catch(Exception e) {
            e.printStackTrace();
            logger.info("Unexpected error");
        }

        ResponseData responseData = new ResponseData(strippedInput, data);
        return new ResponseEntity<>(responseData, response.getStatusCode());
   }

   public MainController(RestTemplateBuilder restTemplateBuilder) {
    this.restTemplate = restTemplateBuilder.build();
   }
}
