<script lang="ts">
	let i = 1;
	let text = "console.log('返回js执行结果完成')";
	let message = `this is the message ${i} from ${text}`;

	async function test1() {
		i++;
		console.log('test1', i);

		let json = await action1();
		message = JSON.stringify(json);
	}

	async function test2() {
		i++;
		console.log('test2', i);

		let json = await action2();
		message = JSON.stringify(json);
	}

	async function test3() {
		i++;
		console.log('test3', i);

		let json = await create_collection();
		message = JSON.stringify(json, null, 2);
	}

	async function test4() {
		i++;
		console.log('test4', i);

		let json = await update_collection();
		message = JSON.stringify(json, null, 2);
	}

	async function test5() {
		i++;
		console.log('test5', i);

		let json = await query();
		message = JSON.stringify(json, null, 2);
	}
	async function test6() {
		i++;
		console.log('test5', i);

		let json = await query_raw();
		message = JSON.stringify(json, null, 2);
	}

	async function action1() {
		let response = await fetch(`http://127.0.0.1:16655/`);
		if (response.status == 200) {
			let json = await response.json();
			return json;
		} else {
			return { state: response.status, message: '404 啦' };
		}
	}

	async function action2() {
		let response = await fetch(`http://127.0.0.1:16655/mmg/hello/ededed`);
		if (response.status == 200) {
			let json = await response.json();
			return json;
		} else {
			return { state: response.status, message: '404 啦' };
		}
	}

	async function update_collection() {
		let collections = [];
		let types = ['Math', 'Physics', 'History'];

		for (let i = 0; i < 10; i++) {
			let book_type = types[(i + 2) % 3];
			let collection = {
				name: `BooK_t_103_${i}`,
				price: 156.5 - i,
				book_type: book_type,
				book_uid: `uid_103_${i}`
			};
			collections.push(collection);
		}

		let data = {
			workspace_id: 'WEB_AAABBB_1001',
			collection_name: 'WebBooks_02',
			collections,
			update_type: 'CreateOnlY'
		};
		let json = await fetch_post('update_collection', data);
		return json;
	}
	import SQL_3 from './raw/Test3.SQL?raw';

	async function query() {
		let params = {
			max_price: 152,
			book_type: 'Math',
			skip: 1
		};

		let query = SQL_3;

		let data = {
			workspace_id: 'WEB_AAABBB_1001',
			query,
			params
		};
		let json = await fetch_post('query', data);
		return json;
	}

	async function query_raw() {
		let params = {
			max_price: 97,
			book_type: 'Physics',
			skip: 1
		};

		let query = SQL_3;

		let data = {
			workspace_id: 'WEB_AAABBB_1001',
			query,
			params
		};
		let json = await fetch_post('query_raw', data);
		return json;
	}

	async function create_collection() {
		let data = {
			workspace_id: 'WEB_AAABBB_1001',
			collection_name: 'WebBooks_02',
			schema: {
				primary_key: 'name',
				indexes_f64: ['price'],
				indexes_string: ['book_type'],
				indexes_string_unique: ['book_uid']
			}
		};
		let json = await fetch_post('create_collection', data);
		return json;
	}

	async function fetch_post(action: string, data: any) {
		let response = await fetch(`http://127.0.0.1:16655/mmg/${action}`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		});

		if (response.status == 200) {
			let json = await response.json();
			return json;
		} else {
			return { state: response.status, message: '404 啦' };
		}
	}
</script>

<div class="test_frame">
	<div>Test3</div>
	<div>测试minimongo</div>
	<div class="test_block">
		<div class="text_title">
			<div>输出结果</div>
			<div>代码</div>
		</div>

		<div class="text_blocks">
			<pre class="message_pre">{message}</pre>
			<textarea class="test_textarea" bind:value={text}></textarea>
		</div>

		<div class="button_block">
			<button type="button" class="test_button" on:click={test1}>连通性测试</button>
			<button type="button" class="test_button" on:click={test2}>连通性测试2</button>
			<div class="test_span">aaa</div>
			<div class="test_span">aaa</div>
		</div>

		<div class="button_block">
			<button type="button" class="test_button" on:click={test3}>create collection</button>
		</div>

		<div class="button_block">
			<button type="button" class="test_button" on:click={test4}>update collection</button>
		</div>

		<div class="button_block">
			<button type="button" class="test_button" on:click={test5}>query</button>
			<button type="button" class="test_button" on:click={test6}>query_raw</button>
		</div>
	</div>
</div>

<style lang="scss">
	.text_title {
		box-sizing: border-box;
		display: flex;
		flex-direction: row;
		margin-bottom: 10px;
		// justify-content: space-between;
		:first-child {
			flex-basis: 50%;
			text-align: center;
		}
		:last-child {
			flex-basis: 50%;
			text-align: center;
		}
	}
	.text_blocks {
		box-sizing: border-box;

		min-height: 600px;
		display: flex;
		flex-direction: row;
		column-gap: 20px;
		// width: 800px;
		// background-color: #09c;
		// position: relative;

		.message_pre {
			box-sizing: border-box;
			border: 1px solid #aaa;
			margin: 0;
			// width: 40%;
			// flex: 1;
			padding: 10px;
			// display: flex;
			flex-basis: 50%;
			overflow: hidden;
			// flex-grow: 0;
			// flex-shrink: 0;
			text-wrap: wrap;
		}
		.test_textarea {
			box-sizing: border-box;
			// min-height: 200px;
			background-color: #6663;
			color: #ddd;
			// width: 50%;
			// flex: 2;
			// padding: 10px;
			// display: flex;
			flex-basis: 50%;
			// flex-grow: 1;
			// flex-shrink: 0;
			padding: 10px;
		}
	}
	.tab_list {
		display: flex;
		flex-direction: row;
		.tab {
			margin: 5px 10px;
			cursor: pointer;
			&.active {
				color: #0099cd;
			}
		}
	}
	.test_frame {
		margin: 20px;
		display: flex;
		flex-direction: column;
	}
	.test_block {
		background-color: #222;
		border: 1px solid #aaa;
		// width: 100%;
		// margin-left: auto;
		// margin-right: auto;
		flex: 1;
		margin-top: 20px;
		display: flex;
		flex-direction: column;
		padding: 20px 20px;
		.test_button {
			margin-top: 20px;
			height: 40px;
			width: 150px;
			background-color: #666;
			color: #ddd;
			font-size: medium;
		}
		.test_text {
			height: 40px;
			margin-top: 20px;
			background-color: #666;
			color: #ddd;
			font-size: large;
			padding: 10px;
		}
		.test_span {
			background-color: #666;
			border: 1px solid #09c;
			display: none;
			height: 60px;
			width: 150px;
		}

		.button_block {
			display: flex;
			flex-direction: row;
			// border: 1px solid #aaa;
			column-gap: 10px;
		}
	}
</style>
