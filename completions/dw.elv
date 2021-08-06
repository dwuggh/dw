
edit:completion:arg-completer[dw] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'dw'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'dw'= {
            cand --generate-shell-completion 'generate shell completion'
            cand -f 'use file'
            cand --file 'use file'
            cand -o 'origin language of the querying text'
            cand --lang-origin 'origin language of the querying text'
            cand -t 'the language to be translated into'
            cand --lang-target 'the language to be translated into'
            cand --format 'response format'
            cand --server 'server mode'
            cand --standalone 'standalone client mode'
            cand --lang-code 'display all available language codes'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
