# 🚀 Учебные CI/CD Pipelines

Коллекция небольших проектов, демонстрирующих полный цикл автоматизации с помощью **GitHub Actions**: проверки кода, тестирование, сборку приложений и публикацию готовых артефактов.

## 📋 Сводная таблица

| # | Проект | Язык | CI | Артефакт | Публикация | Триггер |
|:-:|---|---|---|---|---|---|
| 1 | [hello-rust](./projects/hello-rust) | 🦀 Rust | fmt, Clippy, tests | Docker image | GHCR | push в `main` |
| 2 | [hello-go](./projects/hello-go) | 🐹 Go | gofmt, vet, tests | Docker image | GHCR | push в `main` |
| 3 | [hello-go-releases](./projects/hello-go-releases) | 🐹 Go | gofmt, vet, tests | 5 бинарников | GitHub Releases | тег `v*` |
| 4 | [hello-python-releases](./projects/hello-python-releases) | 🐍 Python | Ruff, pytest | 3 бинарника | GitHub Releases | тег `v*` |
| 5 | [hello-static](./projects/hello-static) | HTML/CSS/JS | HTML/CSS/JS validation | Static site | GitHub Pages | push в `main` |
| 6 | [hello-pages](./projects/hello-pages) | React + Vite | ESLint, build | `dist/` | GitHub Pages | push в `main` |

## Структура

```text
.
├── .github/workflows/
│   ├── validate.yml
│   ├── hello-rust.yml
│   ├── hello-go.yml
│   ├── hello-go-releases.yml
│   ├── hello-python-releases.yml
│   ├── hello-static-pages.yml
│   └── hello-react-pages.yml
├── projects/
│   ├── hello-rust/
│   ├── hello-go/
│   ├── hello-go-releases/
│   ├── hello-python-releases/
│   ├── hello-static/
│   └── hello-pages/
└── README.md
```

## 🐳 Docker images → GHCR

Для Rust и Go workflow выполняет форматирование, статический анализ, тесты и release-сборку. Только после успешного CI образы собираются и отправляются в GitHub Container Registry.

```bash
docker pull ghcr.io/anitouchq/hello-rust:latest
docker run --rm ghcr.io/anitouchq/hello-rust:latest

docker pull ghcr.io/anitouchq/hello-go:latest
docker run --rm ghcr.io/anitouchq/hello-go:latest
```

Публикуемые пакеты можно сделать общедоступными в настройках соответствующего пакета GitHub Packages.

## 📦 GitHub Releases

Go- и Python-приложения собираются под несколько операционных систем. Чтобы создать новую версию, отправьте тег вида `v1.0.0`:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Готовые исполняемые файлы прикладываются к GitHub Release.

## 🌐 GitHub Pages

Два отдельных сценария публикации:

- **Static HTML** — проверки HTML, CSS и JavaScript, затем деплой статических файлов без сборки.
- **React + Vite** — ESLint и production build в CI, передача `dist/` как Pages artifact и публикация только после успешного CI.

Для публикации через Pages выберите **Settings → Pages → Build and deployment → Source: GitHub Actions**.

## 🔐 Разрешения GitHub Actions

- `contents: read` — checkout;
- `packages: write` — публикация в GHCR;
- `contents: write` — создание GitHub Releases;
- `pages: write` и `id-token: write` — GitHub Pages.

Workflow используют встроенный `GITHUB_TOKEN`; дополнительные секреты для GHCR и Releases не требуются.

## 🧪 Проверка

Откройте вкладку **Actions**, чтобы увидеть запуски, логи jobs и статус шагов. Workflow находятся в корневой папке `.github/workflows`, а исходники каждого учебного приложения — в `projects/`.

![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-CI%2FCD-2088FF?logo=githubactions&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Images-2496ED?logo=docker&logoColor=white)
![GHCR](https://img.shields.io/badge/GHCR-Registry-181717?logo=github&logoColor=white)

---

Учебный проект · GitHub Actions · CI/CD
