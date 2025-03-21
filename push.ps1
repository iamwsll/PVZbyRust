# push.ps1 
Write-Host "正在推送所有本地分支到远程仓库..."
git push --all origin

# Write-Host "`n正在同步删除远程分支（本地已删除的分支）..."

# # 获取远程默认分支（如master/main）
# $remoteDefaultBranch = (git remote show origin | Where-Object { $_ -match 'HEAD branch' }) -replace 'HEAD branch: '

# # 获取受保护分支列表（默认分支+自定义）
# $protectedBranches = @('master', 'main', 'develop', $remoteDefaultBranch) | Select-Object -Unique

# # 获取有效远程分支列表
# $remoteBranches = git branch -r |
#     Where-Object { $_ -notmatch ' -> ' } |
#     ForEach-Object { ($_ -split '/', 2)[1].Trim() } |
#     Where-Object { $_ -ne 'HEAD' }

# foreach ($branch in $remoteBranches) {
#     # 跳过受保护分支
#     if ($protectedBranches -contains $branch) {
#         Write-Host "跳过受保护分支: $branch"
#         continue
#     }

#     # 准确检查本地分支是否存在
#     git show-ref --verify --quiet "refs/heads/$branch"
#     if ($LASTEXITCODE -ne 0) {
#         Write-Host "删除远程分支: $branch"
#         git push origin --delete $branch
#     }
# }

Write-Host "`n同步完成！当前分支状态："
git branch -a
