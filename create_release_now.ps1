# Create GitHub Release NOW
$repo = "kisa134/Mycelium"
$tag = "v1.0.0"
$releaseName = "Mycelium v1.0.0 - Symbiosis Protocol"
$releaseBody = Get-Content "RELEASE_DESCRIPTION.md" -Raw

Write-Host "Creating GitHub Release for $repo..."
Write-Host "Tag: $tag"
Write-Host "Name: $releaseName"

# Create release data
$releaseData = @{
    tag_name = $tag
    name = $releaseName
    body = $releaseBody
    draft = $false
    prerelease = $false
} | ConvertTo-Json -Depth 10

Write-Host "Release data prepared..."
Write-Host "Attempting to create release..."

# Try to create release
try {
    $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases" -Method Post -Body $releaseData -Headers @{
        "Authorization" = "token $env:GITHUB_TOKEN"
        "Accept" = "application/vnd.github.v3+json"
        "Content-Type" = "application/json"
    }
    
    Write-Host "✅ Release created successfully!"
    Write-Host "Release ID: $($response.id)"
    Write-Host "Release URL: $($response.html_url)"
    
    # Now upload the asset
    Write-Host "Uploading mycelium-app.exe..."
    $uploadUrl = $response.upload_url -replace "\{.*\}", ""
    $assetUrl = "$uploadUrl?name=mycelium-app.exe&label=Windows%20Executable"
    
    $fileBytes = [System.IO.File]::ReadAllBytes("mycelium-app.exe")
    $assetResponse = Invoke-RestMethod -Uri $assetUrl -Method Post -Body $fileBytes -Headers @{
        "Authorization" = "token $env:GITHUB_TOKEN"
        "Accept" = "application/vnd.github.v3+json"
        "Content-Type" = "application/octet-stream"
    }
    
    Write-Host "✅ Asset uploaded successfully!"
    Write-Host "Asset URL: $($assetResponse.browser_download_url)"
    
} catch {
    Write-Host "❌ Error: $($_.Exception.Message)"
    Write-Host "Make sure GITHUB_TOKEN is set and has repo permissions"
    Write-Host "You can set it with: `$env:GITHUB_TOKEN = 'your_token_here'"
} 