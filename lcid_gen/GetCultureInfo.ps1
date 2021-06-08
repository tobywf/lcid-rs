# Special cases
# =============
#
# Numbered language codes
# -----------------------
#
# When queried by LCID, these map to the deprecated information:
#
# +--------+----------+------------------------------+----------+-----------------------+
# |  LCID  | Old Name | Old EnglishName              | New Name | New EnglishName       |
# +--------+----------+------------------------------+----------+-----------------------+
# | 0x0004 | zh-CHS   | Chinese (Simplified) Legacy  | zh-Hans  | Chinese (Simplified)  |
# | 0x7C04 | zh-CHT   | Chinese (Traditional) Legacy | zh-Hant  | Chinese (Traditional) |
# +--------+----------+------------------------------+----------+-----------------------+
#
# However, when queried by the new name/IETF tag, they also return those LCIDs.
# So I've chosen to use the currently recommended names.
#
# The LCID 0x0086/134 is defined as "qut" in the MS-LCID spec, but gets returned
# as "quc". But "quc" is defined as LCID 0x0093/147 in the spec. When queried by
# name/IETF tag, they both return the former LCID and EnglishName "K'iche'".
# However, the IETF defines "quc" as "Central Quiché" and "qut" as
# "West Central Quiché".
#
# The LCID 0x040A/1034 and name/IETF tag "es-ES_tradnl" return CultureInfos with
# a truncated "es-ES" name/IETF tag. However, "es-ES" is also a valid name that
# maps to LCID 0xc0a/3082. Curiously, the EnglishName for both is different,
# with the former being "Spanish (Spain, Traditional Sort)" and the latter being
# "Spanish (Spain, International Sort)".
#
# On another note, the invariant language LCID 0x007F/127 has an empty name/IETF
# tag, which works fine.
#
# Finally, the LCID 0x4C09/19465 is unsupported when this script was run on
# Windows Server 2019. The corresponding name "en-AE" returns a best-guess
# CultureInfo:
# {
#   "LCID": 4096,
#   "Name": "en-AE",
#   "TwoLetterISOLanguageName": "en",
#   "ThreeLetterISOLanguageName": "eng",
#   "ThreeLetterWindowsLanguageName": "ENU",
#   "EnglishName": "Unknown Locale (en-AE)"
# }
# I believe this should be:
# {
#   "LCID": 19465,
#   "Name": "en-AE",
#   "TwoLetterISOLanguageName": "en",
#   "ThreeLetterISOLanguageName": "eng",
#   "ThreeLetterWindowsLanguageName": "ENU",
#   "EnglishName": "English (United Arab Emirates)"
# }
#
# Named language codes
# --------------------
#
# * "bm-ML" is returned as "bm-Latn-ML"
# * "fa-AF" is mapped to "prs-AF" (including the LCID)
# * "ff-NG" returns the LCID 0x0467/1127, which is "fuv-NG" (reserved), however
#   the name is returned correctly
# * "nus-SD" is mapped to "nus-SS"
# * "swc-CD" is mapped to "sw-CD"
#
# The 16 names return "Unknown Locale": ccp-Cakm-BD, ccp-Cakm-IN, ceb-Latn-PH,
# ff-Latn-BF, ff-Latn-CM, ff-Latn-GH, ff-Latn-GM, ff-Latn-GN, ff-Latn-GW,
# ff-Latn-LR, ff-Latn-MR, ff-Latn-NE, ff-Latn-NG, ff-Latn-SL, ngo-GN, ps-PK.
#
# I have not fixed these up, as it only affects EnglishName, and the three
# letter Windows language name.

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

# fix-up: 0x0004/zh-Hans and 0x7C04/zh-Hant - this happens automatically if the
# information from the name instead of the LCID is used

# fix-up: 0x0086/qut
$QUT_LCID = 0x0086;
$QUT_NAME = "qut";

# fix-up: 0x040A/es-ES_tradnl
$ES_ES_TRADITIONAL_LCID = 0x040A;
$ES_ES_TRADITIONAL_NAME = "es-ES_tradnl";

# fix-up: 0x4C09/en-AE
$EN_AE_LCID = 19465;
$EN_AE = @{
    LCID=$EN_AE_LCID;
    Name="en-AE";
    TwoLetterISOLanguageName="en";
    ThreeLetterISOLanguageName="eng";
    ThreeLetterWindowsLanguageName="ENU";
    EnglishName="English (United Arab Emirates)";
    ANSICodePage=1252;
};

$culture_infos = [ordered]@{};

$numbered = (Get-Content ".\ms-lcid-14-1-numbered.json" | ConvertFrom-Json);
$named = (Get-Content ".\ms-lcid-14-1-named.json" | ConvertFrom-Json);

Write-Host "--- Numbered";

foreach ($value in $numbered) {
    $lcid_hex = $value[0];
    $lcid = ([int32]$lcid_hex);
    $name = $value[1];
    $status = $value[2];

    if ($status -ne "None") {
        continue;
    }

    # CultureInfo from LCID can fail
    try {
        $ci = New-Object System.Globalization.CultureInfo $lcid;
        $lcid_code_page = $ci.TextInfo.ANSICodePage;
        $lcid_ci = $ci | Select-Object $props;
    }
    catch [System.Globalization.CultureNotFoundException] {
        Write-Warning "$($lcid_hex)/$($name) - Unsupported LCID";
        $lcid_ci = $null;
        $lcid_code_page = 0;
    }
    # CultureInfo from name/IETF tag doesn't fail, and will try and produce
    # best-effort information (e.g. zz => LCID: 4096, EnglishName: Unknown Language (zz))
    $ci = New-Object System.Globalization.CultureInfo $name;
    $name_code_page = $ci.TextInfo.ANSICodePage;
    $name_ci = $ci | Select-Object $props;

    # Validation
    if ($lcid_ci -ne $null) {
        foreach ($prop in $props) {
            $lcid_value = $lcid_ci.$prop;
            $name_value = $name_ci.$prop;
            if ($lcid_value -ne $name_value) {
                Write-Warning "$($lcid_hex)/$($name) - Prop '$($prop)' mismatch: '$($lcid_value)' != '$($name_value)'";
            }
        }

        if ($lcid -ne $lcid_ci.LCID) {
            Write-Warning "$($lcid_hex)/$($name) - LCID mismatch: LCID $($lcid) != $($lcid_ci.LCID)";
        }
        if ($name -ne $lcid_ci.Name) {
            Write-Warning "$($lcid_hex)/$($name) - LCID mismatch: Name '$($name)' != '$($lcid_ci.Name)'";
        }
    }

    if ($lcid_code_page -ne $name_code_page) {
        Write-Warning "$($lcid_hex)/$($name) - Code page mismatch: $($lcid_code_page) != $($name_code_page)";
    }

    # fix-up: 0x0086/qut
    if ($lcid -eq $QUT_LCID) {
        $name_ci.Name = $QUT_NAME;
    }

    # fix-up: es-ES_tradnl
    if ($lcid -eq $ES_ES_TRADITIONAL_LCID) {
        $name_ci.Name = $ES_ES_TRADITIONAL_NAME;
    }

    # fix-up: en-AE
    if ($lcid -eq $EN_AE_LCID) {
        $name_ci = $EN_AE;
    }

    $name_ci | Add-Member -MemberType NoteProperty -Name 'ANSICodePage' -Value $name_code_page;

    # Final consistency check
    if ($lcid -ne $name_ci.LCID) {
        Write-Error "$($lcid_hex)/$($name) - Name mismatch: LCID $($lcid) != $($name_ci.LCID)";
    }
    if ($name -ne $name_ci.Name) {
        Write-Error "$($lcid_hex)/$($name) - Name mismatch: Name '$($name)' != '$($name_ci.Name)'";
    }
    if ($culture_infos.Contains($name)) {
        Write-Error "$($lcid_hex)/$($name) - Already defined";
    }

    $culture_infos.Add($name, $name_ci);
}

Write-Host "--- Named";

$NAMED_LCID = 0x1000;

$BM_ML_NAME = "bm-ML";
$FA_AF_NAME = "fa-AF";
$FF_NG_NAME = "ff-NG";
$NUS_SD_NAME = "nus-SD";
$SWC_CD_NAME = "swc-CD";

foreach ($name in $named) {
    # CultureInfo from name/IETF tag doesn't fail, and will try and produce
    # best-effort information (e.g. zz => LCID: 4096, EnglishName: Unknown Language (zz))
    $ci = New-Object System.Globalization.CultureInfo $name;
    $name_code_page = $ci.TextInfo.ANSICodePage;
    $name_ci = $ci | Select-Object $props;

    # Validation
    if ($name -ne $name_ci.Name) {
        Write-Warning "$($NAMED_LCID)/$($name) - Name mismatch: Name '$($name)' != '$($name_ci.Name)'";
    }
    if ($NAMED_LCID -ne $name_ci.LCID) {
        Write-Warning "$($NAMED_LCID)/$($name) - Name mismatch: LCID $($NAMED_LCID) != $($name_ci.LCID)";
    }

    # fix-up: bm-ML
    if ($name -eq $BM_ML_NAME) {
        $name_ci.Name = $BM_ML_NAME;
    }

    # fix-up: fa-AF
    if ($name -eq $FA_AF_NAME) {
        $name_ci.Name = $FA_AF_NAME;
        $name_ci.LCID = $NAMED_LCID;
    }

    # fix-up: ff-NG
    if ($name -eq $FF_NG_NAME) {
        $name_ci.LCID = $NAMED_LCID;
    }

    # fix-up: nus-SD
    if ($name -eq $NUS_SD_NAME) {
        $name_ci.Name = $NUS_SD_NAME;
    }

    # fix-up: swc-CD
    if ($name -eq $SWC_CD_NAME) {
        $name_ci.Name = $SWC_CD_NAME;
    }

    $name_ci | Add-Member -MemberType NoteProperty -Name 'ANSICodePage' -Value $name_code_page;

    # Final consistency check
    if ($name -ne $name_ci.Name) {
        Write-Error "$($NAMED_LCID)/$($name) - Name mismatch: Name '$($name)' != '$($name_ci.Name)'";
    }
    if ($NAMED_LCID -ne $name_ci.LCID) {
        Write-Error "$($NAMED_LCID)/$($name) - Name mismatch: LCID $($NAMED_LCID) != $($name_ci.LCID)";
    }
    if ($culture_infos.Contains($name)) {
        Write-Error "$($NAMED_LCID)/$($name) - Already defined";
    }

    $culture_infos.Add($name, $name_ci);
}

$culture_infos | ConvertTo-Json | Out-File "culture-infos.json" -encoding "UTF8";
