# CI/CD Pipelines

Учебное портфолио из **10 независимых проектов**, демонстрирующих CI/CD с помощью GitHub Actions: проверки кода, тестирование, сборку приложений и публикацию готовых артефактов.

Каждый проект находится в отдельном репозитории. Этот каталог объединяет проекты, их pipeline и опубликованные результаты.

## Список проектов

| # | Проект | Стек | Что выполняет CI/CD | Результат |
|:-:|---|---|---|---|
| 1 | [hello-rust](https://github.com/anitouchq/hello-rust) | 🦀 Rust | fmt, Clippy, unit/integration tests, release build, Docker | [GHCR](https://github.com/users/anitouchq/packages/container/hello-rust) · [Actions](https://github.com/anitouchq/hello-rust/actions) |
| 2 | [hello-go](https://github.com/anitouchq/hello-go) | 🐹 Go | gofmt, vet, tests, build, Docker | [GHCR](https://github.com/users/anitouchq/packages/container/hello-go) · [Actions](https://github.com/anitouchq/hello-go/actions) |
| 3 | [hello-go-releases](https://github.com/anitouchq/hello-go-releases) | 🐹 Go | gofmt, vet, tests, cross-compilation matrix | [Releases](https://github.com/anitouchq/hello-go-releases/releases) · [Actions](https://github.com/anitouchq/hello-go-releases/actions) |
| 4 | [hello-python-releases](https://github.com/anitouchq/hello-python-releases) | 🐍 Python 3.12 | Ruff, pytest, PyInstaller matrix | [Releases](https://github.com/anitouchq/hello-python-releases/releases) · [Actions](https://github.com/anitouchq/hello-python-releases/actions) |
| 5 | [hello-dotnet](https://github.com/anitouchq/hello-dotnet) | 💠 C# / .NET 10 | restore, build, xUnit, self-contained publish | [Releases](https://github.com/anitouchq/hello-dotnet/releases) · [Actions](https://github.com/anitouchq/hello-dotnet/actions) |
| 6 | [hello-gui](https://github.com/anitouchq/hello-gui) | 🎨 Go + Fyne | gofmt, vet, tests, native GUI build matrix | [Releases](https://github.com/anitouchq/hello-gui/releases) · [Actions](https://github.com/anitouchq/hello-gui/actions) |
| 7 | [hex-loader](https://github.com/anitouchq/hex-loader) | 🔎 Go + Fyne | Intel HEX parser, checksum tests, native GUI build | [Releases](https://github.com/anitouchq/hex-loader/releases) · [Actions](https://github.com/anitouchq/hex-loader/actions) |
| 8 | [arduino-manager](https://github.com/anitouchq/arduino-manager) | 🔧 Go + Fyne | Arduino CLI integration, parser tests, native GUI build | [Releases](https://github.com/anitouchq/arduino-manager/releases) · [Actions](https://github.com/anitouchq/arduino-manager/actions) |
| 9 | [hello-static](https://github.com/anitouchq/hello-static) | 🌐 HTML / CSS / JS | HTML/JS checks, Pages artifact, deploy | [Сайт](https://anitouchq.github.io/hello-static/) · [Actions](https://github.com/anitouchq/hello-static/actions) |
| 10 | [hello-pages](https://github.com/anitouchq/hello-pages) | ⚛️ React 18 + Vite | npm CI, lint/validation, production build, deploy | [Сайт](https://anitouchq.github.io/hello-pages/) · [Actions](https://github.com/anitouchq/hello-pages/actions) |

## Docker → GitHub Container Registry

В `hello-rust` и `hello-go` pipeline запускает проверки и тесты, собирает приложение, а затем создаёт и публикует Docker-образ в GHCR. Публикационный job зависит от успешного CI и выполняется при push в `main`.

Пример запуска Rust-образа:

```bash
docker pull ghcr.io/anitouchq/hello-rust:latest
docker run --rm ghcr.io/anitouchq/hello-rust:latest
```

- [Пакет hello-rust](https://github.com/users/anitouchq/packages/container/hello-rust)
- [Пакет hello-go](https://github.com/users/anitouchq/packages/container/hello-go)

## GitHub Releases

В четырёх проектах бинарники собираются под несколько операционных систем:

| Проект | Сборки |
|---|---|
| `hello-go-releases` | Linux amd64/arm64, macOS amd64/arm64, Windows amd64 |
| `hello-python-releases` | Linux, macOS, Windows |
| `hello-dotnet` | Linux x64, Windows x64 |
| `hello-gui`, `hex-loader`, `arduino-manager` | Linux, macOS, Windows |

Публикация запускается отправкой тега версии вида `v1.0.0`. До отправки тега выполняются CI-проверки; затем создаются платформенные сборки и GitHub Release. Ссылки на страницы Releases находятся в таблице проектов.

## GitHub Pages

- **hello-static** — статический сайт на HTML, CSS и JavaScript. CI проверяет файлы, загружает `public/` как Pages artifact и публикует его отдельным job.
- **hello-pages** — React SPA на Vite. CI устанавливает зависимости, запускает проверки и создаёт `dist/`; отдельный job скачивает artifact и публикует сайт.

Оба проекта используют `needs: ci` или `needs: build`, чтобы деплой выполнялся только после успешной проверки.

## Структура отдельных проектов

В каждом репозитории есть собственный README с описанием исходных файлов, локального запуска, CI/CD workflow и ссылками на результаты:

- [hello-rust](https://github.com/anitouchq/hello-rust#readme)
- [hello-go](https://github.com/anitouchq/hello-go#readme)
- [hello-go-releases](https://github.com/anitouchq/hello-go-releases#readme)
- [hello-python-releases](https://github.com/anitouchq/hello-python-releases#readme)
- [hello-dotnet](https://github.com/anitouchq/hello-dotnet#readme)
- [hello-gui](https://github.com/anitouchq/hello-gui#readme)
- [hex-loader](https://github.com/anitouchq/hex-loader#readme)
- [arduino-manager](https://github.com/anitouchq/arduino-manager#readme)
- [hello-static](https://github.com/anitouchq/hello-static#readme)
- [hello-pages](https://github.com/anitouchq/hello-pages#readme)

## Общие принципы workflow

- Минимальные права `GITHUB_TOKEN`: `contents: read` для CI, `packages: write` для GHCR и `contents: write` только для Release job.
- Для GitHub Pages используются `pages: write` и `id-token: write`.
- Сборки и публикации разделены на jobs с явными зависимостями через `needs`.
- Артефакты передаются между jobs через Actions artifacts.
- Pull request запускают проверки, но не публикуют контейнеры, релизы или сайт.

---

Учебное портфолио · 10 проектов · GitHub Actions · CI/CD
