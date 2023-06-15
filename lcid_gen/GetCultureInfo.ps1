$props = @(
    "LCID",
    # IetfLanguageTag is deprecated for Name.
    "Name",
    "TwoLetterISOLanguageName",
    "ThreeLetterISOLanguageName",
    "ThreeLetterWindowsLanguageName",
    # EnglishName is culture-independent. DisplayName is localized.
    "EnglishName"
);

$culture_info_by_lcid_ok = [ordered]@{};
$culture_info_by_lcid_err = [ordered]@{};
$culture_info_by_name_ok = [ordered]@{};
$culture_info_by_name_err = [ordered]@{};

Write-Host "--- Input";

$language_ids = (Get-Content ".\ms-lcid-15.0.json" | ConvertFrom-Json);

Write-Host "--- Query";

foreach ($value in $language_ids) {
    $lcid_hex = $value[0];
    $lcid = ([int32]$lcid_hex);
    $name = $value[1];
    $status = $value[2];

    try {
        $ci = New-Object System.Globalization.CultureInfo $lcid;
        $code_page = $ci.TextInfo.ANSICodePage;
        $ci = $ci | Select-Object $props;
        $ci | Add-Member -MemberType NoteProperty -Name 'ANSICodePage' -Value $code_page;
        if (!$culture_info_by_lcid_ok.Contains($lcid_hex)) {
            $culture_info_by_lcid_ok.Add($lcid_hex, $ci);
        }
    }
    catch [System.Globalization.CultureNotFoundException] {
        $culture_info_by_lcid_err.Add($lcid_hex, "$_");
    }

    if ($name -ne $null) {
        try {
            $ci = New-Object System.Globalization.CultureInfo $name;
            $code_page = $ci.TextInfo.ANSICodePage;
            $ci = $ci | Select-Object $props;
            $ci | Add-Member -MemberType NoteProperty -Name 'ANSICodePage' -Value $code_page;
            $culture_info_by_name_ok.Add($name, $ci);
        }
        catch [System.Globalization.CultureNotFoundException] {
            $culture_info_by_name_err.Add($name, "$_");
        }
    }
}

Write-Host "--- Output";

$culture_info = [ordered]@{
    "culture_info_by_lcid_ok" = $culture_info_by_lcid_ok;
    "culture_info_by_lcid_err" = $culture_info_by_lcid_err;
    "culture_info_by_name_ok" = $culture_info_by_name_ok;
    "culture_info_by_name_err" = $culture_info_by_name_err;
};

$culture_info | ConvertTo-Json | Out-File "culture-info.json" -encoding "UTF8";

Write-Host "--- Finished";
