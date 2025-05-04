# Wait for the backend to be ready
Write-Host "Waiting for backend to be ready..."
while ($true) {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8000/health" -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            break
        }
    } catch {
        Write-Host "Backend not ready yet, waiting..."
        Start-Sleep -Seconds 1
    }
}

Write-Host "Testing health endpoint..."
Invoke-WebRequest -Uri "http://localhost:8000/health" -UseBasicParsing -Verbose

Write-Host "`nTesting versioned health endpoint..."
Invoke-WebRequest -Uri "http://localhost:8000/api/v1/health" -UseBasicParsing -Verbose

Write-Host "`nAll tests completed!" 