# 🚀 CI/CD Pipelines

Учебное портфолио из независимых репозиториев, демонстрирующих CI/CD с помощью **GitHub Actions**: автоматические проверки, тестирование, сборку приложений и публикацию готовых артефактов.

Каждый проект находится в отдельном GitHub-репозитории. Этот репозиторий — общий каталог и точка входа.

## 📋 Список проектов

| # | Проект | Стек | Что делает | Артефакт / публикация | Actions |
|:-:|---|---|---|---|---|
| 1 | [hello-rust](https://github.com/anitouchq/hello-rust) | 🦀 Rust | fmt, Clippy, unit/integration tests, Docker | Docker image → GHCR | [Runs](https://github.com/anitouchq/hello-rust/actions) |
| 2 | [hello-go](https://github.com/anitouchq/hello-go) | 🐹 Go | gofmt, vet, tests, Docker | Docker image → GHCR | [Runs](https://github.com/anitouchq/hello-go/actions) |
| 3 | [hello-go-releases](https://github.com/anitouchq/hello-go-releases) | 🐹 Go | gofmt, vet, tests, matrix build | Бинарники для 5 платформ → GitHub Releases | [Runs](https://github.com/anitouchq/hello-go-releases/actions) |
| 4 | [hello-python-releases](https://github.com/anitouchq/hello-python-releases) | 🐍 Python | Ruff, pytest, PyInstaller | Бинарники для Linux, macOS и Windows → GitHub Releases | [Runs](https://github.com/anitouchq/hello-python-releases/actions) |
| 5 | [hello-static](https://github.com/anitouchq/hello-static) | 🌐 HTML/CSS/JS | Проверка HTML, CSS и JS | Static site → GitHub Pages | [Runs](https://github.com/anitouchq/hello-static/actions) |
| 6 | [hello-pages](https://github.com/anitouchq/hello-pages) | ⚛️ React + Vite | ESLint, production build | `dist/` → GitHub Pages | [Runs](https://github.com/anitouchq/hello-pages/actions) |

## 🧩 Как устроен набор

Каждый репозиторий автономен и содержит собственный исходный код, тесты, workflow и README. Главный каталог не собирает приложения и не выполняет деплой — он объединяет учебные проекты и описывает назначение каждого pipeline.

## 🐳 Docker → GitHub Container Registry

В проектах Rust и Go pipeline последовательно выполняет:

1. Проверку форматирования и статический анализ.
2. Запуск тестов.
3. Сборку приложения.
4. Сборку Docker-образа.
5. Публикацию образа в GHCR после успешного CI при push в `main`.

Пример запуска Rust-образа после публикации:

```bash
docker pull ghcr.io/anitouchq/hello-rust:latest
docker run --rm ghcr.io/anitouchq/hello-rust:latest
```

### Доступ workflow к GHCR

Workflow используют встроенный `GITHUB_TOKEN` с разрешением `packages: write`. Если GitHub возвращает `permission_denied: write_package`, откройте страницу соответствующего пакета → **Package settings → Manage Actions access → Add repository**, выберите репозиторий workflow и выдайте ему роль **Write**. Если пакета ещё нет, сначала проверьте настройки разрешений workflow в **Repository Settings → Actions → General**.

## 📦 GitHub Releases

Go- и Python-приложения собираются под несколько операционных систем. Новый релиз запускается отправкой тега:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Workflow сначала выполняет CI-проверки, затем матричную сборку и публикацию бинарников в GitHub Release.

## 🌐 GitHub Pages

- **Static HTML** — проверка HTML/CSS/JS и публикация файлов сайта без сборки.
- **React + Vite** — ESLint, production build, передача `dist/` как artifact и отдельный job деплоя.

## 🔐 Permissions

Для workflow применяются минимально необходимые разрешения:
- `contents: read` — checkout;
- `packages: write` — публикация контейнеров в GHCR;
- `contents: write` — создание Releases;
- `pages: write` и `id-token: write` — публикация на GitHub Pages.

Для GHCR и Releases используются встроенные токены `GITHUB_TOKEN`; не нужно хранить персональные токены в исходном коде.

## 🛠 Технологии

![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-CI%2FCD-2088FF?logo=githubactions&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Containerization-2496ED?logo=docker&logoColor=white)
![GHCR](https://img.shields.io/badge/GHCR-Container%20Registry-181717?logo=github&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/Go-1.24-00ADD8?logo=go&logoColor=white)
![Python](https://img.shields.io/badge/Python-3.12-3776AB?logo=python&logoColor=white)

---

Учебное портфолио · GitHub Actions · CI/CD
