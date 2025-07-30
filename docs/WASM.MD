# Building for WASM (experimental)

## Create `index.html`

```html
<!doctypehtml><meta charset=utf-8><title>Zen Editor</title><link href=zen_logo.png rel=icon type=image/x-icon><meta content="width=device-width,initial-scale=1" name=viewport><style>body,html{margin:0;padding:0;height:100%;width:100%;overflow:hidden;background:#1e1e1e;color:#fff;font-family:Arial,sans-serif}  canvas{display:block;margin:0;padding:0}  #loading{position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);text-align:center}</style><div id=loading><p>Loading Zen Editor...</div><canvas id=the_canvas_id></canvas>
```

## Run

`cargo install trunk`

then run:

`trunk serve`
