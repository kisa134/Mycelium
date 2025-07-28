# GitHub Release Creation Script
$repo = "kisa134/Mycelium"
$tag = "v1.0.0"
$releaseName = "Mycelium v1.0.0 - Symbiosis Protocol"
$releaseBody = Get-Content "RELEASE_DESCRIPTION.md" -Raw

# Create release
$releaseData = @{
    tag_name = $tag
    name = $releaseName
    body = $releaseBody
    draft = $false
    prerelease = $false
} | ConvertTo-Json

Write-Host "Creating GitHub Release..."
Write-Host "Repository: $repo"
Write-Host "Tag: $tag"
Write-Host "Name: $releaseName"

# You need to set GITHUB_TOKEN environment variable
# $env:GITHUB_TOKEN = "your_github_token_here"

try {
    $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases" -Method Post -Body $releaseData -Headers @{
        "Authorization" = "token $env:GITHUB_TOKEN"
        "Accept" = "application/vnd.github.v3+json"
    }
    
    Write-Host "Release created successfully!"
    Write-Host "Release ID: $($response.id)"
    Write-Host "Release URL: $($response.html_url)"
} catch {
    Write-Host "Error creating release: $($_.Exception.Message)"
    Write-Host "Make sure GITHUB_TOKEN is set"
} 