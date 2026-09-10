package com.example.api;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.time.Instant;

@RestController
@RequestMapping("/api/v1/health")
public class HealthController {

    public record HealthResponse(String status, Instant timestamp) {}

    @GetMapping
    public ResponseEntity<HealthResponse> checkHealth() {
        return ResponseEntity.ok(new HealthResponse("UP", Instant.now()));
    }
}