# Mülkiyet(Ownership) Konusu

Rust'ın zor konularından birisi mülkiyet hakku kurallarıdır(Ownership Rules) Kuralın maddeleri bellidir.

- Her değerin bir sahibi vardır _(Each Value has an owner)_
- Bu değerin herhangi bir zamanda yalnızca 1 sahip olabilir. _(There can only be 1 owner at any point in time for that value)_
- Değerin sahibi kapsam dışına çıkınca değeri “düşürülür” _(When the owner goes out out scope, the value will bir dropped)_