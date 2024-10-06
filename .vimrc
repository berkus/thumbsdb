set colorcolumn=80
highlight ColorColumn ctermbg=red
set makeprg=make
set shiftwidth=2
set enc=utf-8
set fenc=utf-8
set termencoding=utf-8
set nocompatible
set autoindent
set smartindent
set tabstop=2
set expandtab
set textwidth=80
set showmatch
set comments=sl:/*,mb:\ *,elx:\ */
autocmd BufWritePre * %s/\s\+$//e
