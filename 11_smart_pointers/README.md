# 11 — Smart pointers

```bash
cargo run -p smart_pointers
```

## Temas

| Tipo | Para qué | Hilos |
|------|----------|-------|
| `Box<T>` | Un dueño en el heap; tipos recursivos; `Box<dyn Trait>` | ✅ |
| `Deref` | Usar `*x` y los métodos del tipo interior; *deref coercion* (`&MiBox<String>` → `&str`) | — |
| `Rc<T>` | Varios dueños, solo lectura; `Rc::clone` es barato (incrementa un contador) | ❌ |
| `Cell<T>` | Mutabilidad interior para tipos `Copy` (`get`/`set`) | ❌ |
| `RefCell<T>` | Mutabilidad interior; reglas de préstamo comprobadas **en ejecución** (`borrow`, `borrow_mut`, `try_borrow_mut`) | ❌ |
| `Rc<RefCell<T>>` | Estado compartido y mutable en un hilo | ❌ |
| `Weak<T>` | No mantiene vivo el valor; rompe ciclos (`upgrade()` → `Option`) | — |
| `Cow<'a, T>` | Prestado hasta que hace falta modificar | — |
| `Arc<T>`, `Mutex<T>` | Las versiones para varios hilos (módulo 12) | ✅ |

## Para recordar

- Empieza por referencias normales y valores propios; recurre a `Rc<RefCell<…>>` solo cuando de verdad haya varios dueños.
- `RefCell` traslada los errores de préstamo de compilación a **panics** en ejecución.
- Un ciclo de `Rc` es una fuga de memoria (segura, pero fuga): usa `Weak` para los "punteros hacia atrás".
