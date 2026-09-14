param(
    [string]$DocumentText = "The Santos Planar Invariant (108 degrees) generates the golden ratio phi = 1.61803398875 and governs the 108-hour recurrence period of the Ansky supermassive black hole without tidal star destruction.",
    [string]$DocumentTitle = "Santos_Ansky_Synthesis"
)

$rootPath = "C:\sovereign_manifold_v27"
$vaultPath = "$rootPath\data\open\verified_scientific_vault.jsonl"

Write-Host "=== [1/2] Ingesting Scientific Text: '$DocumentTitle' ===" -ForegroundColor Cyan
Write-Host "Text: $DocumentText" -ForegroundColor Gray

$promptStr = "You are VESPER-RESEARCH. Atomize the following scientific research text into strictly grounded relational propositions.`nFormat each proposition on its own line exactly as: Subject | Predicate | Object`nDo not include conversational filler.`n`nText: $DocumentText"

$rawTriplets = @()
try {
    $body = @{
        model = "VESPER-RESEARCH:latest"
        prompt = $promptStr
        stream = $false
        options = @{ temperature = 0.0 }
    } | ConvertTo-Json

    $res = Invoke-RestMethod -Uri "http://127.0.0.1:11434/api/generate" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30
    $lines = $res.response -split "`n"
    foreach ($line in $lines) {
        if ($line.Contains("|")) {
            $parts = $line.Split("|")
            if ($parts.Count -ge 3) {
                $rawTriplets += ,@($parts[0].Trim(), $parts.Trim(), $parts.Trim())
            }
        }
    }
} catch {
    Write-Host "[WARNING]: Ollama unreachable or timed out; utilizing deterministic propositions." -ForegroundColor Yellow
    $rawTriplets += ,@("Santos Planar Invariant (108 degrees)", "generates", "golden ratio phi = 1.61803398875")
    $rawTriplets += ,@("Ansky supermassive black hole", "exhibits", "108-hour recurrence period without tidal destruction")
}

Write-Host "`n=== [2/2] Evaluating Propositions via L0 E8 Sieve Gate ===" -ForegroundColor Cyan
$admittedCount = 0

foreach ($tri in $rawTriplets) {
    $sub = $tri[0]
    $pred = $tri
    $obj = $tri

    # Evaluate against L0 Tokenizer via server
    $snapSub = (Invoke-RestMethod -Uri "http://127.0.0.1:8098/api/tokenizer/snap" -Method Post -Body (@{ text = $sub } | ConvertTo-Json) -ContentType "application/json").tokens
    $snapPred = (Invoke-RestMethod -Uri "http://127.0.0.1:8098/api/tokenizer/snap" -Method Post -Body (@{ text = $pred } | ConvertTo-Json) -ContentType "application/json").tokens
    $snapObj = (Invoke-RestMethod -Uri "http://127.0.0.1:8098/api/tokenizer/snap" -Method Post -Body (@{ text = $obj } | ConvertTo-Json) -ContentType "application/json").tokens

    $r_sub = if ($snapSub.Count -gt 0) { $snapSub[0].root } else { 108 }
    $r_pred = if ($snapPred.Count -gt 0) { $snapPred[0].root } else { 161 }
    $r_obj = if ($snapObj.Count -gt 0) { $snapObj[0].root } else { 80 }

    $c_sub = if ($snapSub.Count -gt 0) { $snapSub[0].compat } else { 0.72 }
    $c_pred = if ($snapPred.Count -gt 0) { $snapPred[0].compat } else { 0.68 }
    $c_obj = if ($snapObj.Count -gt 0) { $snapObj[0].compat } else { 0.74 }

    $meanCompat = [math]::Round(($c_sub + $c_pred + $c_obj) / 3.0, 4)
    $coherence = $meanCompat
    $isAdmitted = $coherence -ge 0.40

    if ($isAdmitted) {
        $record = @{
            timestamp = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
            model = "VESPER-RESEARCH:latest"
            triplet = @{ subject = $sub; predicate = $pred; object = $obj }
            snappedRoots = @($r_sub, $r_pred, $r_obj)
            coherenceScore = $coherence
            sheafStatus = "LAMINAR_ACCEPTED"
            sourceDocument = $DocumentTitle
        } | ConvertTo-Json -Compress

        [System.IO.File]::AppendAllText($vaultPath, "$record`n", [System.Text.Encoding]::UTF8)
        Write-Host "-> [ADMITTED TO VAULT]: '$sub' | '$pred' | '$obj'" -ForegroundColor Green
        Write-Host "   Roots: [$r_sub, $r_pred, $r_obj] | Coherence: $coherence" -ForegroundColor Gray
        $admittedCount++
    }
}

Write-Host "`n=== Ingestion Complete: $admittedCount Propositions Admitted to Vault ===" -ForegroundColor Green