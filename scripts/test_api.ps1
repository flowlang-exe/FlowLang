# FlowLang Notes API Test Script
# Run the server first: .\target\release\flowlang.exe run .\examples\full_demo.flow

$baseUrl = "http://localhost:3000"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FlowLang Notes API - Route Tests" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Homepage
Write-Host "[1] GET / - Homepage" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    Write-Host "  Content-Type: $($response.Headers['Content-Type'])" -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 2: List Notes
Write-Host "[2] GET /api/notes - List Notes" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/notes" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    Write-Host "  Response: $($response.Content)" -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 3: Create Note (POST)
Write-Host "[3] POST /api/notes - Create Note" -ForegroundColor Yellow
try {
    $body = '{"title": "Test Note", "content": "This is a test note from PowerShell"}'
    $response = Invoke-WebRequest -Uri "$baseUrl/api/notes" -Method POST -Body $body -ContentType "application/json" -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) Created" -ForegroundColor Green
    Write-Host "  Response: $($response.Content)" -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 4: List Notes Again (should have 1 note now)
Write-Host "[4] GET /api/notes - List Notes (after create)" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/notes" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    Write-Host "  Response: $($response.Content)" -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 5: Server Info
Write-Host "[5] GET /api/info - Server Info" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/info" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    $json = $response.Content | ConvertFrom-Json
    Write-Host "  Platform: $($json.Platform) / $($json.arch)" -ForegroundColor Gray
    Write-Host "  CWD: $($json.cwd)" -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 6: Stats
Write-Host "[6] GET /api/stats - Request Stats" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/stats" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    $json = $response.Content | ConvertFrom-Json
    Write-Host "  Total Requests: $($json.totalRequests)" -ForegroundColor Gray
    Write-Host "  Uptime: $($json.uptimeSec)s" -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 7: URL Test
Write-Host "[7] GET /api/url-test - URL Parsing Demo" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/url-test" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    Write-Host "  Response: $($response.Content.Substring(0, [Math]::Min(100, $response.Content.Length)))..." -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 8: Crypto Demo
Write-Host "[8] GET /api/crypto-demo - Crypto Demo" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/crypto-demo" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    $json = $response.Content | ConvertFrom-Json
    Write-Host "  Input: $($json.input)" -ForegroundColor Gray
    Write-Host "  MD5: $($json.md5)" -ForegroundColor Gray
    Write-Host "  SHA256: $($json.sha256.Substring(0, 32))..." -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 9: Debug
Write-Host "[9] GET /api/debug - Request Debug" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/debug" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode) OK" -ForegroundColor Green
    $json = $response.Content | ConvertFrom-Json
    Write-Host "  Method: $($json.method)" -ForegroundColor Gray
    Write-Host "  Path: $($json.pathname)" -ForegroundColor Gray
    Write-Host "  IP: $($json.ip)" -ForegroundColor Gray
} catch {
    Write-Host "  FAILED: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""

# Test 10: 404 Not Found
Write-Host "[10] GET /nonexistent - 404 Test" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/nonexistent" -Method GET -UseBasicParsing
    Write-Host "  Status: $($response.StatusCode)" -ForegroundColor Yellow
} catch {
    if ($_.Exception.Response.StatusCode -eq 404) {
        Write-Host "  Status: 404 Not Found (Expected)" -ForegroundColor Green
    } else {
        Write-Host "  Status: $($_.Exception.Response.StatusCode)" -ForegroundColor Yellow
    }
}
Write-Host ""

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  All Tests Complete!" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
