# Decorator vs. Adapter vs. Proxy vs. Facade
All ot them are **structural** design patterns and all ot them **wrap** types.<br>
They differ in what interface they provide to a client:
- **Proxy** wraps types and provides **the same interface** to a client.
- **Decorator** /ˈdekəreɪtər/ wraps types and **extends interface** to a client.
- **Adapter** /əˈdæptər/ wraps types and **changes interface** providing to a client **another interface**, i.e. the **adapter** pattern just **links two incompatible interfaces**.
- **Facade** /fəˈsɑːd/ wraps types and **changes interface** providing to a client **simplified interface**.

<br>

So, the purpose of a **facade** is **simplicity** and the purpose of an **adapter** is **interoperability**.