from PIL import Image

# 2つの画像ファイルパスを指定
image1_path = 'samples/sample_outputs/sample1.png'
image2_path = 'samples/sample_outputs/sample2.png'

# 画像を開く
image1 = Image.open(image1_path)
image2 = Image.open(image2_path)

# 画像のサイズを取得
width1, height1 = image1.size
width2, height2 = image2.size

# 結合後の画像の幅と高さを計算
new_width = width1 + width2
new_height = max(height1, height2)  # 高さは2つの画像のうち最大のものを選ぶ

# 結合後の画像を作成
new_image = Image.new('RGB', (new_width, new_height))

# 画像を結合
new_image.paste(image1, (0, 0))
new_image.paste(image2, (width1, 0))

# 結合した画像を保存
new_image.save('combined_image.jpg')
