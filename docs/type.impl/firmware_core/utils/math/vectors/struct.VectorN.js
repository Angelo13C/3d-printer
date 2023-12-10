(function() {var type_impls = {
"firmware_core":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#166-239\">source</a><a href=\"#impl-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.ZERO\" class=\"associatedconstant\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#176\">source</a><h4 class=\"code-header\">pub const <a href=\"firmware_core/utils/math/vectors/struct.VectorN.html#associatedconstant.ZERO\" class=\"constant\">ZERO</a>: Self = _</h4></section></summary><div class=\"docblock\">\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"macro\">assert_eq!</span>(VectorN::&lt;<span class=\"number\">2</span>&gt;::ZERO.length_millimeters(), <span class=\"number\">0.</span>);\n<span class=\"macro\">assert_eq!</span>(VectorN::&lt;<span class=\"number\">3</span>&gt;::ZERO.length_millimeters(), <span class=\"number\">0.</span>);\n<span class=\"macro\">assert_eq!</span>(VectorN::&lt;<span class=\"number\">4</span>&gt;::ZERO.length_millimeters(), <span class=\"number\">0.</span>);\n<span class=\"comment\">// ...</span></code></pre></div>\n</div></details><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#178-181\">source</a><h4 class=\"code-header\">pub const fn <a href=\"firmware_core/utils/math/vectors/struct.VectorN.html#tymethod.new\" class=\"fn\">new</a>(components: [<a class=\"struct\" href=\"firmware_core/utils/measurement/distance/struct.Distance.html\" title=\"struct firmware_core::utils::measurement::distance::Distance\">Distance</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">N</a>]) -&gt; Self</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.length_millimeters_sqr\" class=\"method\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#196-201\">source</a><h4 class=\"code-header\">pub fn <a href=\"firmware_core/utils/math/vectors/struct.VectorN.html#tymethod.length_millimeters_sqr\" class=\"fn\">length_millimeters_sqr</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a></h4></section></summary><div class=\"docblock\"><p>Returns the square of the length of this vector in millimeters.</p>\n<h5 id=\"examples\"><a href=\"#examples\">Examples</a></h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>x = Distance::from_millimeters(<span class=\"number\">3</span>);\n<span class=\"kw\">let </span>y = Distance::from_millimeters(<span class=\"number\">4</span>);\n\n<span class=\"kw\">let </span>vector2 = VectorN::&lt;<span class=\"number\">2</span>&gt;::from_xy(x, y);\n\n<span class=\"macro\">assert_eq!</span>(vector2.length_millimeters_sqr() <span class=\"kw\">as </span>i32, x.as_millimeters().sqr() + y.as_millimeters().sqr());</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.length_millimeters\" class=\"method\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#216-219\">source</a><h4 class=\"code-header\">pub fn <a href=\"firmware_core/utils/math/vectors/struct.VectorN.html#tymethod.length_millimeters\" class=\"fn\">length_millimeters</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a></h4></section></summary><div class=\"docblock\"><p>Returns the length of this vector in millimeters.</p>\n<h5 id=\"examples-1\"><a href=\"#examples-1\">Examples</a></h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>x = Distance::from_millimeters(<span class=\"number\">3</span>);\n<span class=\"kw\">let </span>y = Distance::from_millimeters(<span class=\"number\">4</span>);\n\n<span class=\"kw\">let </span>vector2 = VectorN::&lt;<span class=\"number\">2</span>&gt;::from_xy(x, y);\n\n<span class=\"macro\">assert_eq!</span>(vector2.length_millimeters(), vector2.length_millimeters_sqr().sqrt());</code></pre></div>\n</div></details><section id=\"method.normalized\" class=\"method\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#221-224\">source</a><h4 class=\"code-header\">pub fn <a href=\"firmware_core/utils/math/vectors/struct.VectorN.html#tymethod.normalized\" class=\"fn\">normalized</a>(&amp;self) -&gt; Self</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.dot\" class=\"method\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#229-238\">source</a><h4 class=\"code-header\">pub fn <a href=\"firmware_core/utils/math/vectors/struct.VectorN.html#tymethod.dot\" class=\"fn\">dot</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>) -&gt; <a class=\"struct\" href=\"firmware_core/utils/measurement/distance/struct.Distance.html\" title=\"struct firmware_core::utils::measurement::distance::Distance\">Distance</a></h4></section></summary><div class=\"docblock\"><p>Returns the <a href=\"https://en.wikipedia.org/wiki/Dot_product\"><code>dot product</code></a> of this vector with <code>other</code>.</p>\n</div></details></div></details>",0,"firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#158-164\">source</a><a href=\"#impl-Default-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#160-163\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#impl-Debug-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<section id=\"impl-StructuralEq-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#impl-StructuralEq-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralEq.html\" title=\"trait core::marker::StructuralEq\">StructuralEq</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section>","StructuralEq","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Neg-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#123-135\">source</a><a href=\"#impl-Neg-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h4></section></summary><div class='docblock'>The resulting type after applying the <code>-</code> operator.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.neg\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#127-134\">source</a><a href=\"#method.neg\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg\" class=\"fn\">neg</a>(self) -&gt; Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output\" title=\"type core::ops::arith::Neg::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the unary <code>-</code> operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg\">Read more</a></div></details></div></details>","Neg","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<section id=\"impl-Eq-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#impl-Eq-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section>","Eq","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#impl-PartialEq-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#239\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MulAssign%3Cf32%3E-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#147-156\">source</a><a href=\"#impl-MulAssign%3Cf32%3E-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html\" title=\"trait core::ops::arith::MulAssign\">MulAssign</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>&gt; for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.mul_assign\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#149-155\">source</a><a href=\"#method.mul_assign\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign\" class=\"fn\">mul_assign</a>(&amp;mut self, rhs: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>)</h4></section></summary><div class='docblock'>Performs the <code>*=</code> operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign\">Read more</a></div></details></div></details>","MulAssign<f32>","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Sub%3C%26VectorN%3CN%3E%3E-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#110-122\">source</a><a href=\"#impl-Sub%3C%26VectorN%3CN%3E%3E-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;&amp;<a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;&gt; for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h4></section></summary><div class='docblock'>The resulting type after applying the <code>-</code> operator.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.sub\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#114-121\">source</a><a href=\"#method.sub\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub\" class=\"fn\">sub</a>(self, rhs: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>) -&gt; Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output\" title=\"type core::ops::arith::Sub::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the <code>-</code> operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub\">Read more</a></div></details></div></details>","Sub<&VectorN<N>>","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#impl-Clone-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Add%3C%26VectorN%3CN%3E%3E-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#96-108\">source</a><a href=\"#impl-Add%3C%26VectorN%3CN%3E%3E-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;<a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;&gt; for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h4></section></summary><div class='docblock'>The resulting type after applying the <code>+</code> operator.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#100-107\">source</a><a href=\"#method.add\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add\" class=\"fn\">add</a>(self, rhs: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>) -&gt; Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output\" title=\"type core::ops::arith::Add::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the <code>+</code> operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add\">Read more</a></div></details></div></details>","Add<&VectorN<N>>","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IndexMut%3Cusize%3E-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#88-94\">source</a><a href=\"#impl-IndexMut%3Cusize%3E-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#90-93\">source</a><a href=\"#method.index_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut\" class=\"fn\">index_mut</a>(&amp;mut self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>) -&gt; &amp;mut Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" title=\"type core::ops::index::Index::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the mutable indexing (<code>container[index]</code>) operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut\">Read more</a></div></details></div></details>","IndexMut<usize>","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Mul%3Cf32%3E-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#137-146\">source</a><a href=\"#impl-Mul%3Cf32%3E-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>&gt; for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h4></section></summary><div class='docblock'>The resulting type after applying the <code>*</code> operator.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.mul\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#141-145\">source</a><a href=\"#method.mul\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul\" class=\"fn\">mul</a>(self, rhs: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>) -&gt; Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output\" title=\"type core::ops::arith::Mul::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the <code>*</code> operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul\">Read more</a></div></details></div></details>","Mul<f32>","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<section id=\"impl-StructuralPartialEq-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#76\">source</a><a href=\"#impl-StructuralPartialEq-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section>","StructuralPartialEq","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Index%3Cusize%3E-for-VectorN%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#79-87\">source</a><a href=\"#impl-Index%3Cusize%3E-for-VectorN%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"firmware_core/utils/math/vectors/struct.VectorN.html\" title=\"struct firmware_core::utils::math::vectors::VectorN\">VectorN</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"struct\" href=\"firmware_core/utils/measurement/distance/struct.Distance.html\" title=\"struct firmware_core::utils::measurement::distance::Distance\">Distance</a></h4></section></summary><div class='docblock'>The returned type after indexing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/firmware_core/utils\\math/vectors.rs.html#83-86\">source</a><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index\" class=\"fn\">index</a>(&amp;self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>) -&gt; &amp;Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" title=\"type core::ops::index::Index::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the indexing (<code>container[index]</code>) operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index\">Read more</a></div></details></div></details>","Index<usize>","firmware_core::utils::math::vectors::Vector2","firmware_core::utils::math::vectors::Vector3"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()