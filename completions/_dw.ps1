
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'dw' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'dw'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'dw' {
            [CompletionResult]::new('--generate-shell-completion', 'generate-shell-completion', [CompletionResultType]::ParameterName, 'generate shell completion')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'use file')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'use file')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'origin language of the querying text')
            [CompletionResult]::new('--lang-origin', 'lang-origin', [CompletionResultType]::ParameterName, 'origin language of the querying text')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'the language to be translated into')
            [CompletionResult]::new('--lang-target', 'lang-target', [CompletionResultType]::ParameterName, 'the language to be translated into')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'response format')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--server', 'server', [CompletionResultType]::ParameterName, 'server mode')
            [CompletionResult]::new('--standalone', 'standalone', [CompletionResultType]::ParameterName, 'standalone client mode')
            [CompletionResult]::new('--lang-code', 'lang-code', [CompletionResultType]::ParameterName, 'display all available language codes')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
