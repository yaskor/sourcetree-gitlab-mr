# sourcetree-gitlab-mr

A small utility to simplify the creation of GitLab Merge Requests directly from Sourcetree. Written in Rust.

---

## ✨ What does it do?

`sourcetree-gitlab-mr` opens Google Chrome with the GitLab Merge Request page, pre-filled with the current Git branch and repository currently open in Sourcetree.

---

## 💡 Why was this built?

Jira has announced that it will no longer support GitLab self-managed instances for Merge Requests integration. 

This tool was created to quickly open a GitLab Merge Request in the browser.

This is a Rust port of:  
https://github.com/JCID/sourcetree-custom-action-gitlab-merge-request-opener  
The reason for the port is to avoid requiring PHP/Composer.  

---

## 🖥️ Requirements

- Works only on **Windows**
- Requires **Google Chrome** installed

---

## ⚙️ Setup in Sourcetree

1. Go to **Tools** > **Options** > **Custom Actions**.
2. Click **Add** to create a new custom action.
3. Fill in the following:

   | Field              | Value                                      |
   |--------------------|--------------------------------------------|
   | Menu Caption       | GitLab Merge Request                        |
   | Script to run      | Path to the `sourcetree-gitlab-mr.exe`      |
   | Parameters         | `$REPO $BRANCH`                             |

4. Done! You can now right-click in Sourcetree and select **GitLab Merge Request** to open the MR page.

---

## ✅ Features

- Opens the MR creation page directly
- Automatically uses the current branch
- Works without additional dependencies like PHP or Composer
- Fast and native (Rust-powered)