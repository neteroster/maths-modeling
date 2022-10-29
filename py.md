```python
points = np.arange(-5, 5, 0.01)
xs, ys = np.meshgrid(points, points)
```

生成两个个类似坐标系网格矩阵，非常适合画图

```python
z = xs ** 2 + ys ** 2 + xs * ys
plt.imshow(z, cmap=plt.cm.gray, extent=[-5, 5, -5, 5])
```
